import { Bar, HistoryMetadata, LibrarySymbolInfo, PeriodParams } from "../../../charting_library/datafeed-api";

import { getErrorMessage, logMessage, RequestParams, UdfErrorResponse, UdfOkResponse, UdfResponse } from "./helpers";

import { IRequester } from "./irequester";
import ReconnectingWebSocket from "reconnecting-websocket";
import { JSONRPCClient } from "json-rpc-2.0";

// tslint:disable: no-any
interface HistoryPartialDataResponse extends UdfOkResponse {
    t: any;
    c: any;
    o?: never;
    h?: never;
    l?: never;
    v?: never;
}

interface HistoryFullDataResponse extends UdfOkResponse {
    t: any;
    c: any;
    o: any;
    h: any;
    l: any;
    v: any;
}
// tslint:enable: no-any
interface HistoryNoDataResponse extends UdfResponse {
    s: "no_data";
    nextTime?: number;
}

type HistoryResponse = HistoryFullDataResponse | HistoryPartialDataResponse | HistoryNoDataResponse;

export type PeriodParamsWithOptionalCountback = Omit<PeriodParams, "countBack"> & { countBack?: number };

export interface GetBarsResult {
    bars: Bar[];
    meta: HistoryMetadata;
}

export interface LimitedResponseConfiguration {
    /**
     * Set this value to the maximum number of bars which
     * the data backend server can supply in a single response.
     * This doesn't affect or change the library behavior regarding
     * how many bars it will request. It just allows this Datafeed
     * implementation to correctly handle this situation.
     */
    maxResponseLength: number;
    /**
     * If the server can't return all the required bars in a single
     * response then `expectedOrder` specifies whether the server
     * will send the latest (newest) or earliest (older) data first.
     */
    expectedOrder: "latestFirst" | "earliestFirst";
}

export class HistoryProvider {
    private _datafeedUrl: string;
    private readonly _requester: IRequester;
    private readonly _limitedServerResponse?: LimitedResponseConfiguration;
    _client: JSONRPCClient<void>;

    public constructor(
        datafeedUrl: string,
        requester: IRequester,
        limitedServerResponse?: LimitedResponseConfiguration
    ) {
        this._datafeedUrl = datafeedUrl;
        this._requester = requester;
        this._limitedServerResponse = limitedServerResponse;

        const ws = new ReconnectingWebSocket(`ws://127.0.0.1:8080/ws`, [], { debug: false });

        const client = new JSONRPCClient((request) => {
            try {
                ws.send(JSON.stringify(request));
                return Promise.resolve();
            } catch (error) {
                return Promise.reject(error);
            }
        });

        ws.addEventListener("open", function open() {
            // this will only be called on every reconnect attempt
            client.rejectAllPendingRequests("reconnect");
        });

        ws.addEventListener("message", (event: MessageEvent<any>) => {
            //console.log("======== received", json);
            client.receive(JSON.parse(event.data));
        });

        ws.addEventListener("close", () => {
            client.rejectAllPendingRequests("close");
        });

        this._client = client;
    }

    public getBars(
        symbolInfo: LibrarySymbolInfo,
        resolution: string,
        periodParams: PeriodParamsWithOptionalCountback
    ): Promise<GetBarsResult> {
        logMessage(`HistoryProvider: getBars for ${symbolInfo} ${resolution} ${periodParams}`);
        const requestParams: RequestParams = {
            symbol: symbolInfo.ticker || "",
            resolution: resolution,
            from: periodParams.from,
            to: periodParams.to,
            use_local: (globalThis as any).use_local ?? false,
        };
        if (periodParams.countBack !== undefined) {
            requestParams.countback = periodParams.countBack;
        }

        if (symbolInfo.currency_code !== undefined) {
            requestParams.currencyCode = symbolInfo.currency_code;
        }

        if (symbolInfo.unit_id !== undefined) {
            requestParams.unitId = symbolInfo.unit_id;
        }

        return new Promise(async (resolve: (result: GetBarsResult) => void, reject: (reason: string) => void) => {
            try {
                let initialResponse: HistoryResponse = await this._client.request("history", [requestParams]);
                //console.warn(initialResponse);

                /*
                const initialResponse = await this._requester.sendRequest<HistoryResponse>(
                    this._datafeedUrl,
                    "history",
                    requestParams
                );
                //*/

                const result = this._processHistoryResponse(initialResponse);

                if (this._limitedServerResponse) {
                    await this._processTruncatedResponse(result, requestParams);
                }
                resolve(result);
            } catch (e: unknown) {
                if (e instanceof Error || typeof e === "string") {
                    const reasonString = getErrorMessage(e);
                    // tslint:disable-next-line:no-console
                    console.warn(`HistoryProvider: getBars() failed ${e}, error=${reasonString}`);
                    reject(reasonString);
                }
            }
        });
    }

    private async _processTruncatedResponse(result: GetBarsResult, requestParams: RequestParams) {
        let lastResultLength = result.bars.length;
        try {
            while (
                this._limitedServerResponse &&
                this._limitedServerResponse.maxResponseLength > 0 &&
                this._limitedServerResponse.maxResponseLength === lastResultLength &&
                requestParams.from < requestParams.to
            ) {
                // adjust request parameters for follow-up request
                if (requestParams.countback) {
                    requestParams.countback = (requestParams.countback as number) - lastResultLength;
                }
                if (this._limitedServerResponse.expectedOrder === "earliestFirst") {
                    requestParams.from = Math.round(result.bars[result.bars.length - 1].time / 1000);
                } else {
                    requestParams.to = Math.round(result.bars[0].time / 1000);
                }

                const followupResponse = await this._requester.sendRequest<HistoryResponse>(
                    this._datafeedUrl,
                    "history",
                    requestParams
                );
                const followupResult = this._processHistoryResponse(followupResponse);
                lastResultLength = followupResult.bars.length;
                // merge result with results collected so far
                if (this._limitedServerResponse.expectedOrder === "earliestFirst") {
                    if (followupResult.bars[0].time === result.bars[result.bars.length - 1].time) {
                        // Datafeed shouldn't include a value exactly matching the `to` timestamp but in case it does
                        // we will remove the duplicate.
                        followupResult.bars.shift();
                    }
                    result.bars.push(...followupResult.bars);
                } else {
                    if (followupResult.bars[followupResult.bars.length - 1].time === result.bars[0].time) {
                        // Datafeed shouldn't include a value exactly matching the `to` timestamp but in case it does
                        // we will remove the duplicate.
                        followupResult.bars.pop();
                    }
                    result.bars.unshift(...followupResult.bars);
                }
            }
        } catch (e: unknown) {
            /**
             * Error occurred during followup request. We won't reject the original promise
             * because the initial response was valid so we will return what we've got so far.
             */
            if (e instanceof Error || typeof e === "string") {
                const reasonString = getErrorMessage(e);
                // tslint:disable-next-line:no-console
                console.warn(`HistoryProvider: getBars() warning during followup request, error=${reasonString}`);
            }
        }
    }

    private _processHistoryResponse(response: HistoryResponse | UdfErrorResponse) {
        if (response.s !== "ok" && response.s !== "no_data") {
            throw new Error(response.errmsg);
        }

        const bars: Bar[] = [];
        const meta: HistoryMetadata = {
            noData: false,
        };

        if (response.s === "no_data") {
            meta.noData = true;
            meta.nextTime = response.nextTime;
        } else {
            const volumePresent = response.v !== undefined;
            const ohlPresent = response.o !== undefined;

            for (let i = 0; i < response.t.length; ++i) {
                const barValue: Bar = {
                    time: response.t[i] * 1000,
                    close: parseFloat(response.c[i]),
                    open: parseFloat(response.c[i]),
                    high: parseFloat(response.c[i]),
                    low: parseFloat(response.c[i]),
                };

                if (ohlPresent) {
                    barValue.open = parseFloat((response as HistoryFullDataResponse).o[i]);
                    barValue.high = parseFloat((response as HistoryFullDataResponse).h[i]);
                    barValue.low = parseFloat((response as HistoryFullDataResponse).l[i]);
                }

                if (volumePresent) {
                    barValue.volume = parseFloat((response as HistoryFullDataResponse).v[i]);
                }

                bars.push(barValue);
            }
        }

        return {
            bars: bars,
            meta: meta,
        };
    }
}

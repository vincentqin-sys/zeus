"use client";
import React from "react";
import GridLayout from "react-grid-layout";
import { StockChart } from "../widgets/charts/stock";
import { ResolutionString } from "@/public/static/charting_library/charting_library";
import { StockOptionTable } from "../stock/stock_option_table";
import { UDFCompatibleDatafeed } from "@/public/static/datafeeds/udf/src/udf-compatible-datafeed";

interface Props {}

export const MainGrid = (props: Props) => {
    return (
        <>
            <div className="grid grid-cols-12 bg-white h-full">
                {/* left grids */}
                <div className="col-span-8 grid grid-cols-8 grid-rows-10">
                    <div className="row-span-5 col-span-4 border-solid border-2 border-slate-300" key="1-1">
                        <StockChart
                            resolution={"5" as ResolutionString}
                            macd_config={[{ fast: 4, slow: 9, signal: 9 }]}
                            hidden_extra_toolbar={true}
                            standalone={false}
                        />
                    </div>
                    <div className="row-span-5 col-span-4 border-solid border-2 border-slate-300" key="1-4">
                        <StockChart
                            resolution={"1" as ResolutionString}
                            macd_config={[{ fast: 4, slow: 9, signal: 9 }]}
                            hidden_extra_toolbar={true}
                            standalone={false}
                        />
                    </div>

                    <div className="row-span-5 col-span-4 border-solid border-2 border-slate-300" key="1-3">
                        <StockChart
                            resolution={"15" as ResolutionString}
                            macd_config={[{ fast: 4, slow: 9, signal: 9 }]}
                            hidden_extra_toolbar={true}
                            standalone={false}
                        />
                    </div>
                    <div className="row-span-5 col-span-4 border-solid border-2 border-slate-300" key="1-2">
                        <StockChart
                            resolution={"3" as ResolutionString}
                            macd_config={[{ fast: 4, slow: 9, signal: 9 }]}
                            hidden_extra_toolbar={false}
                            standalone={true}
                        />
                    </div>
                </div>
                {/* right grids */}
                <div className="col-span-4 grid grid-rows-10">
                    <div key="2-1" className="row-span-5 border-solid border-2 border-slate-300">
                        <StockChart
                            resolution={"1D" as ResolutionString}
                            macd_config={[{ fast: 4, slow: 9, signal: 9 }]}
                            hidden_extra_toolbar={true}
                            standalone={false}
                        />
                    </div>
                    <div key="2-3" className="row-span-5 border-solid border-2 border-slate-300">
                        <StockChart
                            resolution={"60" as ResolutionString}
                            macd_config={[{ fast: 4, slow: 9, signal: 9 }]}
                            hidden_extra_toolbar={true}
                            standalone={false}
                        />
                    </div>
                </div>
            </div>
        </>
    );
};

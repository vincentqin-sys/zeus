import os
import time
from loguru import logger
from selenium import webdriver
from selenium.webdriver.chrome.options import Options


def excute(dir, lines):
    chrome_options = Options()
    chrome_options.add_argument("--headless")
    driver = webdriver.Chrome(chrome_options)
    driver.set_window_size(1920, 1080)

    for idx, l in enumerate(lines[1:]):
        l = l.split()
        path = f"./data/{dir}/{idx:04d}-{l[0]}.png"
        logger.debug(f"{idx} {l} {os.path.isfile(path)}")

        if os.path.isfile(path):
            continue
        driver.get(f"http://localhost:3000/local?symbolState=%22{l[0]}%22")
        time.sleep(8)

        driver.save_screenshot(path)
    driver.quit()

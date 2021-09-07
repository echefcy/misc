from selenium import webdriver
from selenium.webdriver.common.keys import Keys
import time

driver = webdriver.Firefox()
driver.get("https://play.typeracer.com/")


def fun():
    input_box = driver.find_element_by_css_selector(".txtInput")
    txt = driver.find_elements_by_css_selector("span[unselectable]")
    tt = ""
    print(len(txt))
    if len(txt) > 2:
        for t in range(len(txt)):
            print(txt[t].text)
            tt += (txt[t].text)
            if t == 1:
                tt += " "
    elif len(txt) <= 2:
        for t in range(len(txt)):
            print(txt[t].text)
            tt += (txt[t].text)
            if t == 0:
                tt += " "

    print(tt)

    for letter in tt:
        time.sleep(0.07)
        input_box.send_keys(letter)


while True:
    i = input(": ")
    fun()

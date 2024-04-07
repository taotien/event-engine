from settings import CLIENT_DOMAIN
import requests
import json


class Client:

    def __init__(self):
        self.__domain = CLIENT_DOMAIN
        self.__headers = {
            "Content-Type": "application/json"
        }

    def push(self, data):
        res = requests.post(url=self.__domain + "/add", headers=self.__headers, data=json.dumps(data))
        print(f"push: {res.status_code}")

    def fetch(self):
        res = requests.get(url=self.__domain + "/list")
        print(f"fetch: {res.status_code}")


if __name__ == '__main__':
    c = Client()
    c.push()

import json
import logging

import requests

from settings import CLIENT_DOMAIN

logging.basicConfig(level=logging.INFO,
                    format='%(filename)s %(asctime)s - %(name)s - %(levelname)s - %(message)s',
                    filename='crawler.log', filemode='a')


class Client:
    """
    Client for communicating with DB backend service.
    """

    def __init__(self):
        self.__domain = CLIENT_DOMAIN
        self.__headers = {
            "Content-Type": "application/json"
        }
        self.__logger = logging.getLogger(__name__)

    def push(self, data):
        """
        Push event data to DB

        Args:
            data(json): single event data in json format

        Return:
            None
        """
        res = requests.post(url=self.__domain + "/add", headers=self.__headers, data=json.dumps(data))
        self.__logger.info(f"push: {res.status_code}")

    def fetch(self):
        """
        Fetch event datas from DB, get a list of events.

        Return:
            None
        """
        res = requests.get(url=self.__domain + "/list")
        self.__logger.info(f"fetch: {res.status_code}")

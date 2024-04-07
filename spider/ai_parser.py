import os

from openai import OpenAI


class AIParser:
    def __init__(self):
        self.__api = os.environ.get('OPENAI_API_KEY')
        self.__client = OpenAI(api_key=self.__api)

    def parse(self, content):
        res = self.__client.chat.completions.create(
                  model="gpt-3.5-turbo-0125",
                  response_format={"type": "json_object"},
                  messages=[
                        {"role": "system",
                                 "content": "You will extract and summarize data in JSON format from the content I sent you."},
                        {"role": "user",
                                 "content": """summarize content in JSON format like this
                                            {
                                                "events": {
                                                    1: {
                                                      "name": "",
                                                      "start_time": "",
                                                      "end_time": "",
                                                      "location": "",
                                                      "description": "",
                                                      "check_list": [],
                                                      "price": "0",
                                                      "tags": [],
                                                      "source": ""
                                                    },
                                                    2: {
                                                      "name": "",
                                                      "start_time": "",
                                                      "end_time": "",
                                                      "location": "",
                                                      "description": "",
                                                      "check_list": [],
                                                      "price": "0",
                                                      "tags": [],
                                                      "source": ""
                                                    },
                                                    // ALL FIELD FORMAT should be in STRING
                                                    // keep going if there are more events in the same page
                                                    // leave all fields as EMPTY STRING if not found
                                                    // check_list is a list with 5 elements, imagine 5 things we should prepare for based on the description 
                                                    // default price is "0", leave it as "0" if not found, price should be STRING
                                                    // try to get the location as detailed as possible
                                                    // location should be STRING ONLY
                                                    // tags are related labels you summarized based on the content
                                                    // source will always be the one I send in the content
                                                    // all time are in unix timestamp format
                                                }
                                            }
                                            CONTENT TO SUMMARIZE AND TO EXTRACT:  %s""" % content
                        }
                  ]
                )
        return res.choices[0].message.content

import json
import os
import sys
from datetime import datetime

from google.auth.transport.requests import Request
from google.oauth2.credentials import Credentials
from google_auth_oauthlib.flow import InstalledAppFlow
from googleapiclient.discovery import build


class CalendarParser:
    def __init__(self):
        self.__creds = None
        self.__service = None
        self.__event = None
        self.setup()

    def setup(self):
        # Set up Google Calendar API access
        SCOPES = ['https://www.googleapis.com/auth/calendar']
        self.__creds = None

        # The file token.json stores the user's access and refresh tokens
        if os.path.exists('token.json'):
            self.__creds = Credentials.from_authorized_user_file('token.json')
        # If there are no (valid) credentials available, let the user log in
        if not self.__creds or not self.__creds.valid:
            if self.__creds and self.__creds.expired and self.__creds.refresh_token:
                self.__creds.refresh(Request())
            else:
                flow = InstalledAppFlow.from_client_secrets_file(
                    'credentials.json', SCOPES)
                self.__creds = flow.run_local_server(port=0)
            # Save the credentials for the next run
            with open('token.json', 'w') as token:
                token.write(self.__creds.to_json())

        # Build the Google Calendar service
        self.__service = build('calendar', 'v3', credentials=self.__creds)

    def parse(self, data):
        check_list = "\n".join([f"({str(idx+1)})" + item for idx, item in enumerate(data.get("check_list"))])
        desc = f'[DESCRIPTION]\n{data.get("description")}\n[CHECK_LIST]\n{check_list}\n[PRICE]\n{data.get("price")}\n[SOURCE]\n{data.get("source")}\n'

        start_time = data.get('start_time')
        end_time = data.get('end_time')

        # Parse the string into a datetime object
        start_date_time = datetime.strptime(start_time, '%Y,%m,%d,%H,%M,%S')
        start_timestamp = int(start_date_time.timestamp())

        end_date_time = datetime.strptime(end_time, '%Y,%m,%d,%H,%M,%S')
        end_timestamp = int(end_date_time.timestamp())


        # Create the event
        self.__event = {
            'summary': data.get('name', ''),
            'location': data.get('location', ''),
            'description': desc,
            'start': {
                'dateTime': datetime.utcfromtimestamp(start_timestamp).isoformat() + 'Z',
                'timeZone': 'America/Los_Angeles',
            },
            'end': {
                'dateTime': datetime.utcfromtimestamp(end_timestamp).isoformat() + 'Z',
                'timeZone': 'America/Los_Angeles',
            },
        }

    def register(self):
        # Call the API to insert the event
        self.__event = self.__service.events().insert(calendarId='primary', body=self.__event).execute()
        print('Event created: %s' % (self.__event.get('htmlLink')))


if __name__ == '__main__':
    if len(sys.argv) != 2:
        print("Usage: python3 cal.py <json-format-str>")
        exit(1)
    cal = CalendarParser()
    datas = sys.argv[1]

    for data in datas:
        cal.parse(data)
        cal.register()







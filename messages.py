from json import loads
from threading import Thread
from time import sleep
import websocket
from auth import User
from ferrumec import AbstractUser


class MessagesUser(AbstractUser):
    def __init__(self, user: User) -> None:
        super().__init__(user)
        self.BASE_URL = "http://localhost/messages"
        self.BUS_URL = "ws://localhost:8082"

    def create_conversation(self, participants):
        r = self.session.post(
            f"{self.BASE_URL}/conversations",
            json={"participants": participants},
        )
        print(r.status_code, r.content)
        if r.ok:
            return (r.json())["name"]

        return r.status_code, r.content

    def send_message(self, conv, text):
        self.session.post(
            f"{self.BASE_URL}/conversations/{conv}/messages",
            json={"text": text},
        )

    def fetch_messages(self, conv):
        with self.session.get(
            f"{self.BASE_URL}/conversations/{conv}/messages",
        ) as r:
            return r.json()

    def fetch_receipts(self, message):
        with self.session.get(
            f"{self.BASE_URL}/messages/{message}/receipts",
        ) as r:
            return r.json()

    def ws_client(self, handler):
        def on_message(ws, message):
            handler(loads(message))

        def on_connect(ws):
            print("connected", self.user.username)

        ws = websocket.WebSocketApp(
            f"{self.BUS_URL}/ws/",
            header={"Authorization": f"Bearer {self.user.access}"},
            on_message=on_message,
            on_open=on_connect,
        )
        ws.run_forever()


if __name__ == "__main__":
    user = User("alice")
    user2 = User("bob")

    messanger = MessagesUser(user)
    messanger2 = MessagesUser(user2)
    Thread(target=messanger2.ws_client, args=(print,)).start()
    conv = messanger.create_conversation([user2.username])
    messanger.send_message(conv, "Hi there")
    sleep(2)
    msgs = messanger.fetch_messages(conv)
    print(msgs)

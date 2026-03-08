from json import loads
from threading import Thread
from time import sleep, time
import websocket
from auth import User, requests
from ferrumec import AbstractUser

SENT_AT = 0
RECEIVED_AFTER = 0


class MessagesUser(AbstractUser):
    def __init__(self, user: User) -> None:
        super().__init__(user)
        self.BASE_URL = "http://localhost:8080/messages"
        self.BUS_URL = "ws://localhost:8080/messages"

    def create_conversation(self, participants):
        r = self.session.post(
            f"{self.BASE_URL}/conversations",
            json={"participants": participants},
        )
        if r.ok:
            return (r.json())["name"]

        return r.status_code, r.content

    def send_message(self, conv, text):
        self.session.post(
            f"{self.BASE_URL}/conversations/{conv}/messages",
            json={"text": text},
        )
    @staticmethod
    def get_userid(username):
        res = requests.get(f"http://localhost:8080/auth/user_id/username/{username}")
        return res.content.decode()

    def send_pmessage(self, peer, text):
        peer = MessagesUser.get_userid(peer)
        print('peer-userid',peer)
        self.post(f"/inbox/{peer}/messages", json={"text": text})

    def fetch_inbox(self):
        return self.get("/inbox/messages")

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

        print("trying to connect ws")
        ws = websocket.WebSocketApp(
            f"{self.BUS_URL}/ws/",
            header={"Authorization": f"Bearer {self.user.access}"},
            on_message=on_message,
            on_open=on_connect,
            on_error=print,
        )
        th = Thread(target=ws.run_forever)
        th.start()


def p2p_latency(msg):
    global RECEIVED_AFTER
    now = time()
    RECEIVED_AFTER = now - SENT_AT
    print(msg)
    print(f"p2p latency:{RECEIVED_AFTER*1000}ms")


if __name__ == "__main__":
    user = User("alice")
    user2 = User("bob")

    messanger = MessagesUser(user)
    messanger2 = MessagesUser(user2)
    messanger2.ws_client(
        p2p_latency,
    )
    SENT_AT = time()
    messanger.send_pmessage("bob", "Hi there")
    sleep(2)
    msgs = messanger2.fetch_inbox()
    receipts = messanger.fetch_receipts(msgs[0]["id"])
    print(msgs)
    print(receipts)

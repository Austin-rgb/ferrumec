import requests


class RequestError(Exception):
    pass


def resp(func):
    def wrapper(*args, **kwargs):
        res = func(*args, **kwargs)
        if res.ok:
            try:
                return res.json()
            except:
                return res.content
        raise RequestError(f"[{res.status_code}]{res.content}")

    return wrapper


class LoginFailed(Exception):
    pass


class RegisterFailed(Exception):
    pass


class User:
    BASE_URL = "http://localhost:8080/auth"

    def __init__(self, username, password="password123") -> None:
        self.error = None
        if not self.login(username, password):
            User.register(username)
            print("registration success")
            if not self.login(username, password):
                raise self.error  # type: ignore

    def login(self, username, password):
        res = requests.post(
            f"{self.BASE_URL}/auth/login",
            json={"username": username, "password": password},
            timeout=10,
        )
        if res.ok:
            resobj = res.json()
            self.username = username
            self.access = resobj["data"]["access_token"]
            self.refresh = resobj["data"]["refresh_token"]
            print("login success")

        else:
            self.error = LoginFailed(f"{res.status_code}:{res.content}")
        return res.ok

    @staticmethod
    def register(username, password="password123"):
        res = requests.post(
            f"{User.BASE_URL}/auth/register",
            json={"username": username, "password": password},
            timeout=10,
        )
        if not res.ok:
            raise RegisterFailed(f"{res.status_code}:{res.content}")

    def __repr__(self) -> str:
        return f"User({self.username})"


class PasswdLess(User):
    def __init__(self, token) -> None:
        res = PasswdLess.register_confirm(token)
        self.access = res["access_token"]
        self.refresh = res["refresh_token"]

    @staticmethod
    def register_start(email):
        return requests.post(
            User.BASE_URL + "/passwordless/register/start", json={"email": email}
        )

    @staticmethod
    @resp
    def register_confirm(token):
        return requests.get(
            User.BASE_URL + f"/passwordless/register/confirm_link/{token}"
        )

    @staticmethod
    @resp
    def challange(user_id):
        return requests.post(
            User.BASE_URL + f"/passwordless/challange/{user_id}",
        )

    @staticmethod
    @resp
    def confirm(token):
        return requests.get(User.BASE_URL + f"/passwordless/confirm_link/{token}")

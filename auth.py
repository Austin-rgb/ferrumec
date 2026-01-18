import requests


class LoginFailed(Exception):
    pass


class RegisterFailed(Exception):
    pass


class User:
    BASE_URL = "http://localhost/auth"

    def __init__(self, username, password="password123") -> None:
        res = requests.post(
            f"{self.BASE_URL}/auth/login",
            json={"username": username, "password": password},
            timeout=10,
        )
        if res.ok:
            res = res.json()
            self.username = username
            self.access = res["data"]["access_token"]
            self.refresh = res["data"]["refresh_token"]

        else:
            print("[login] status:", res.status_code, res.content)
            raise LoginFailed(res.status_code)

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

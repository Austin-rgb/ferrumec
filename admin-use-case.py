from typing import Callable
import requests


class RequestError(Exception):
    pass


URL = "http://localhost:8080"


def resp(func: Callable[..., requests.Response]):  # -> Callable[..., Response]:
    def wrapper(*args, **kwargs):
        res = func(*args, **kwargs)
        if res.ok:
            return res.json()

        raise RequestError(f"{res.request.path_url}=>[{res.status_code}]{res.content}")

    return wrapper


class Admin:

    def __init__(self) -> None:
        self.session = requests.Session()

    @resp
    def login(self):
        data = {"username": "ostiness", "password": "ostiness"}
        res = self.session.post(URL + "/auth/auth/admin/login", json=data)
        token = res.json().get("token")
        print("token", token)
        # self.session.headers["Authorization"] = f"Bearer {token}"
        self.session.cookies.set("access_token", token)
        return res

    @resp
    def grant(self, perm, target):
        return self.session.post(
            URL + "/permissions/grant",
            json={"permission": perm, "target": target, "aud": "*"},
        )

    @resp
    def post_product(self, **details):
        return self.session.post(URL + "/catalog/products", json=details)


admin = Admin()
print(admin.login())
print(
    admin.post_product(
        name="mouse", price=19.99, category="accessories", qty=1, sku="12345xyz"
    )
)
print(admin.grant("create_product", "alice"))

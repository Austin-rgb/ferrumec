from curses.ascii import US
from auth import User
import requests


class TenantUser:
    def __init__(self, user: User) -> None:
        self.user = user
        self.url = "http://localhost:8080"
        self.session = requests.Session()
        self.session.headers["Authorization"] = f"Bearer {user.access}"

    def add_member(self, member):
        res = self.session.post(f"{self.url}/add_member", json={"name": member})
        return res.json()

    def add_permission(self, perm):
        res = self.session.post(f"{self.url}/add_permission", json={"name": perm})
        return res.json()

    def grant_permission(self, perm, target):
        res = self.session.post(
            f"{self.url}/grant_permission", json={"permission": perm, "target": target}
        )
        return res.json()

    def deny_permission(self, perm, target):
        res = self.session.post(
            f"{self.url}/deny_permission", json={"permission": perm, "target": target}
        )
        return res.json()


class TenantMember(TenantUser):
    def __init__(self, user: User, tenant: str) -> None:
        super().__init__(user)
        self.tt = None
        self.tenant = tenant
        self.refresh()

    def refresh(self):
        res = self.session.post(
            f"{self.url}/tenant_token", json={"tenant": self.tenant}
        )
        if not res.ok:
            print(res.status_code, res.content)

        self.tt = res.json()["token"]
        self.session.headers["Tenant-Token"] = f"Bearer {self.tt}"

    def read(self):
        res = self.session.post(f"{self.url}/team/read")
        print(res.status_code, res.content)

    def write(self):
        res = self.session.post(f"{self.url}/team/write")
        print(res.status_code, res.content)


if __name__ == "__main__":
    user = User("alice")
    user2 = User("bob")
    tu = TenantUser(user)
    try:
        tm = TenantMember(user2, "alice")
    except:
        print("tm failed")
    print(tu.add_member("bob"))
    tm = TenantMember(user2, "alice")
    tm = TenantMember(user2, "alice")
    tm.read()
    print(tu.add_permission("read"))
    tm.read()
    print(tu.grant_permission("read", "bob"))
    tm.refresh()
    tm.read()

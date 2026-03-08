from attr import dataclass
from auth import User
from ferrumec import AbstractUser


@dataclass
class Product:
    name: str
    description: str
    price: float
    category: str


class CatalogUser(AbstractUser):
    def __init__(self, user: User) -> None:
        super().__init__(user)
        self.BASE_URL = self.BASE_URL + "/catalog"

    def post_product(self, product: Product):
        res = self.post("/products", json=product.__dict__)
        if not res.ok:
            return res.status_code, res.content
        return res.json()

    def get_product(self, product: int):
        return self.get(f"/products/{product}")

    def get_by_slug(self, slug: str):
        res = self.get(f"/products/slug/{slug}")
        return res.json()

    def get_products(self, query: dict):
        return self.get("/products", params=query)


if __name__ == "__main__":
    alice = User("alice")
    user = CatalogUser(alice)
    product = Product("new product", "my very new product", 123, "new")
    post = user.post_product(product)
    print(post)
    get = user.get_product(post["id"])  # type: ignore
    get = user.get_by_slug(post["slug"])  # type: ignore
    products = user.get_products({})
    print("products", products)

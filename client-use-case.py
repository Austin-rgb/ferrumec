from auth import PasswdLess, User
from catalog import CatalogUser
from messages import MessagesUser
from orders import OrdersUser


URL = "http://localhost:8080"


class Client:
    def __init__(self, user: User) -> None:
        self.catalog = CatalogUser(user)
        self.orders = OrdersUser(user)
        messages = MessagesUser(user)
        messages.ws_client(print)
        self.messages = messages


username = "a@b.c"
# submit identity and contact information
r = PasswdLess.register_start(username).content
print("pswdless-register-start", r)
# user = PasswdLess(input("enter token"))
user = User("alice")
client = Client(user)
# search for a product with a general name
# client.post_product(name="mouse", price=19.99, category="accessories")
products = client.catalog.get_products(query=dict(q="mous"))
print(products)
# filter products

# get product details
product = client.catalog.get_product(products[0]["id"])
print(product)
# add product to cart

# submit identity and contact information

# confirm contact information

# submit order
order = client.orders.create([(product["id"], 1)])
print(order)
# submit delivery information

# make payments
orders = client.orders.get_orders()
print("[Orders]")
print(orders)
# check order status

# get order confirmation

# check delivery status

# get delivery notification

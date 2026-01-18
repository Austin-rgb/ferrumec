from time import sleep
from auth import User
from messages import MessagesUser

# User.register("alice")
user = User("alice")

messanger = MessagesUser(user)
conv = messanger.create_conversation(["bob"])
messanger.send_message(conv, "Hi there")
sleep(2)
msgs = messanger.fetch_messages(conv)
print(msgs)

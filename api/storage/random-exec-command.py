import os
import random
import threading

commands = [
    "ls -l",
    "pwd",
    "whoami",
    "date",
    "uname -a",
    "uptime",
    "df -h",
    "free -m",
    "ps aux",
    "hostname",
]

def run_random_commands():
    repeat = random.randint(1000, 10000)
    for _ in range(repeat):
        command = random.choice(commands)
        os.system(command)

threads = []

# Number of threads you want
num_threads = 10

for _ in range(num_threads):
    t = threading.Thread(target=run_random_commands)
    t.start()
    threads.append(t)

for t in threads:
    t.join()

print("All random command simulations finished.")
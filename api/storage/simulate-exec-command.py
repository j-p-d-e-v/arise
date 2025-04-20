import random
import threading
import subprocess
import shlex

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
    "id",
    "who",
    "w",
    "top -b -n1",
    "netstat -tunlp",
    "dmesg | tail",
    "vmstat",
    "iostat",
    "lscpu",
    "lsblk",
    "uptime",
    "uname -r",
    "uname -m",
    "whoami",
    "groups",
    "cat /etc/os-release",
    "lsof -i",
    "ss -tuln",
    "ip a",
    "ip r",
    "traceroute google.com",
    "ping -c 1 8.8.8.8",
    "curl -I https://google.com",
    "head -n 5 /etc/passwd",
    "tail -n 5 /etc/passwd",
    "du -sh /tmp",
    "df -i",
    "find /tmp -type f | wc -l",
    "history | tail -n 5",
    "uptime -p"
]

def run_random_commands():
    repeat = random.randint(1000, 30000)
    for _ in range(repeat):
        command = random.choice(commands)
        try:
            # safer splitting of command into list
            subprocess.run(shlex.split(command), stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        except Exception as e:
            pass
            #print(f"Failed to run command: {command}\nError: {e}")

threads = []

# Number of threads
num_threads = 10

for _ in range(num_threads):
    t = threading.Thread(target=run_random_commands)
    t.start()
    threads.append(t)

for t in threads:
    t.join()

print("All random command simulations finished.")

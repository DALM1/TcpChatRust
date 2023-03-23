import socket

HOST = '127.0.0.1'  # Adresse IP du serveur
PORT = 5000  # Port à utiliser

# Mot de passe pour se connecter au serveur
PASSWORD = 'my_secret_password'


USERNAME = None


sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)


sock.bind((HOST, PORT))


sock.listen()

print(f"Le serveur écoute sur {HOST}:{PORT}")

while True:
   
    print("En attente de connexion...")
    conn, addr = sock.accept()
    print(f"Connexion acceptée de {addr[0]}:{addr[1]}")

   
    password = conn.recv(1024).decode().strip()

   
    if password != PASSWORD:
        conn.send("Mot de passe incorrect".encode())
        conn.close()
        continue

    
    conn.send("Bienvenue au serveur !".encode())

   
    username = conn.recv(1024).decode().strip()
    USERNAME = username

    print(f"{USERNAME} a rejoint le chat.")

    while True:
        
        data = conn.recv(1024)

        if not data:
            break

        message = data.decode().strip()

        if message == "/quit":
           

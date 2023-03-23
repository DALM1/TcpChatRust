import socket

HOST = '127.0.0.1'  # Adresse IP du serveur
PORT = 5000  # Port à utiliser

# Mot de passe pour se connecter au serveur
PASSWORD = 'my_secret_password'

# Nom d'utilisateur connecté
USERNAME = None

# Création d'une socket TCP/IP
sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

# Association de la socket à l'adresse IP et au port
sock.bind((HOST, PORT))

# Mise en écoute de la socket
sock.listen()

print(f"Le serveur écoute sur {HOST}:{PORT}")

while True:
    # Attente d'une connexion
    print("En attente de connexion...")
    conn, addr = sock.accept()
    print(f"Connexion acceptée de {addr[0]}:{addr[1]}")

    # Lecture du mot de passe envoyé par le client
    password = conn.recv(1024).decode().strip()

    # Vérification du mot de passe
    if password != PASSWORD:
        conn.send("Mot de passe incorrect".encode())
        conn.close()
        continue

    # Envoi d'un message de bienvenue
    conn.send("Bienvenue au serveur !".encode())

    # Lecture du nom d'utilisateur envoyé par le client
    username = conn.recv(1024).decode().strip()
    USERNAME = username

    print(f"{USERNAME} a rejoint le chat.")

    while True:
       
        data = conn.recv(1024)

        if not data:
            break

        message = data.decode().strip()

        if message == "/quit":
           

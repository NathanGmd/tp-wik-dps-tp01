# Serveur HTTP en Rust

Ce projet implémente un serveur HTTP en Rust qui écoute sur un port spécifié par la variable d'environnement `PORT` et accepte des requêtes. Lorsqu'une requête `GET` est envoyée à l'endpoint `/ping`, le serveur répond avec un statut HTTP 200 et renvoie les informations de la requête sous forme de JSON.

## Prérequis

Avant de commencer, assurez-vous que vous avez **Rust** et **Cargo** installés sur votre machine.

- Si vous n'avez pas encore installé Rust, vous pouvez le faire en suivant les instructions ici : [Installer Rust](https://www.rust-lang.org/learn/get-started).

## Installation et Exécution

### Étape 1 : Cloner ce dépôt

Clonez ce dépôt Git sur votre machine locale :

```bash
git clone https://github.com/NathanGmd/tp-wik-dps-tp01.git
cd tp-wik-dps-tp01
```
### Étape 2 : Configurer la variable d'environnement PORT

Le serveur utilise la variable d'environnement PORT pour déterminer le port sur lequel il écoute. Si vous ne définissez pas cette variable, le serveur ne se lancera pas.

#### - Sur Linux/macOS :
```
export PORT=3000
```
#### - Sur Windows (Command Prompt) :
```
set PORT=3000
```
#### - Sur Windows (PowerShell) :
```
$env:PORT=3000
```
### Étape 3 : Lancer le serveur

Après avoir configuré la variable PORT, vous pouvez démarrer le serveur en exécutant la commande suivante dans votre terminal :
```
cargo run
```
Le serveur démarrera et écoutera sur le port que vous avez configuré.

## Tester le Serveur

### Vérification de la réponse

Une fois le serveur en fonctionnement, vous pouvez tester l'endpoint /ping en envoyant une requête GET via curl :
```
curl http://localhost:3000/ping
```
Le serveur renverra une réponse avec un statut HTTP 200 et le contenu JSON de la requête.

#### Exemple de réponse réussie :
```
{
  "GET": "/ping",
  "Host": "localhost:3000",
  "User-Agent": "curl/7.79.1",
  "Accept": "*/*"
}
```
## Tester une requête incorrecte

Si vous envoyez une requête vers un endpoint qui n'est pas /ping, vous recevrez une réponse 404 Not Found :
```
curl http://localhost:3000/foo
```
### Réponse :
```
HTTP/1.1 404 NOT FOUND
Content-Length: 0
```
## Structure du code

- main() : Le serveur est initialisé et écoute sur le port spécifié dans la variable d'environnement PORT.
- handle_connection(stream: TcpStream) : Cette fonction gère les connexions entrantes et génère la réponse en fonction de la requête reçue.
- http_request_to_json(request: Vec<String>) : Cette fonction convertit les en-têtes de la requête HTTP en un format JSON pour la réponse.

## Deploiement sur docker

### Creation de l'image Docker

```
FROM rust:latest AS builder
ENV HOME=/home/root
WORKDIR $HOME/rapi
RUN mkdir -p src && echo "fn main() {}" > src/main.rs
COPY Cargo.toml Cargo.lock ./
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/home/root/rapi/target \
    cargo fetch
COPY src src
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/home/root/rapi/target \
    cargo build --release && \
    cp target/release/tp-wik-dps-tp01 ./app
FROM debian:latest
RUN useradd -m -d /app rapi_user
USER rapi_user
WORKDIR /app
ENV PORT=3002
COPY --from=builder /home/root/rapi/app .
EXPOSE $PORT
ENTRYPOINT ["./app"]
```
Buildkit de build une image avec les dépendance directement compilées dans le cache, ce qui permet d'avoir une image beaucoup plus légére et un déploiement beaucoup plus rapide !

L'image est ici créer en deux stages, le premier stage crée une image en cache avec les dépendances compilées ainsi qu'un main.rs vide afin que la compilation puisse avoir lieux.
Le deuxième crée un user pour que l'application n'est pas accès au root du docker. Ensuite le port est définie. Et enfin le fichier local est copié dans le docker dans le /app, si les dépendances ne sont pas modifiées, elles ne sont pas recompilées ce qui accelère le deploiement de l'app car uniquement le code modifié est recompilé.
A noter que si le code n'est pas modifié, il n'est pas recompilé non plus. 

### Build de l'image
#### Lancer la construction de l'image :

```
docker build -t rust-api --progress=plain .
```

#### Lancement du docker

```
docker run -p 8085:3002 rust-api
```

Les résultats seront les même qu'avec le serveur lancé en local.

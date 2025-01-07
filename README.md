# Serveur HTTP en Rust

Ce projet implémente un serveur HTTP en Rust qui écoute sur un port spécifié par la variable d'environnement `PORT` et accepte des requêtes. Lorsqu'une requête `GET` est envoyée à l'endpoint `/ping`, le serveur répond avec un statut HTTP 200 et renvoie les informations de la requête sous forme de JSON.

## Prérequis

Avant de commencer, assurez-vous que vous avez **Rust** et **Cargo** installés sur votre machine.

- Si vous n'avez pas encore installé Rust, vous pouvez le faire en suivant les instructions ici : [Installer Rust](https://www.rust-lang.org/learn/get-started).

## Installation et Exécution

### Étape 1 : Cloner ce dépôt

Clonez ce dépôt Git sur votre machine locale :

```bash
git clone https://github.com/mon-utilisateur/mon-repo.git
cd mon-repo

```
### Étape 2 : Configurer la variable d'environnement PORT

Le serveur utilise la variable d'environnement PORT pour déterminer le port sur lequel il écoute. Si vous ne définissez pas cette variable, le serveur écoutera par défaut sur le port 3000.

## - Sur Linux/macOS :
```
export PORT=3000
```
## - Sur Windows (Command Prompt) :
```
set PORT=3000
```
## - $env:PORT=3000
```
$env:PORT=3000
```
### Étape 3 : Lancer le serveur

Après avoir configuré la variable PORT, vous pouvez démarrer le serveur en exécutant la commande suivante dans votre terminal :
```
cargo run
```
Le serveur démarrera et écoutera sur le port que vous avez configuré.

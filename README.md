```
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

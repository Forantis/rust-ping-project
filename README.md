# Ping Project

Ce projet est un petit serveur HTTP en Rust qui écoute sur un port configurable et retourne les headers de la requête au format JSON lorsqu'une requête GET est effectuée sur `/ping`, et retourne une erreur 404 sur toute autre route.

## Prérequis

- Rust (https://www.rust-lang.org/tools/install)


## Installation

Clonez le dépôt et naviguez dans le répertoire du projet :

```sh
git clone <URL_DU_DEPOT>
cd ping-project
```

## Configuration et lancement
```sh
export PING_LISTEN_PORT=8080  # Optionnel, pour définir le port d'écoute au
cargo run
```

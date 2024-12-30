
# ToDo List CLI - Apprentissage de Rust

## Description

Ce projet est une application en ligne de commande (CLI) développée en **Rust**. L'objectif principal est d'apprendre les concepts fondamentaux du langage Rust à travers un projet concret. L'application permet de gérer une liste de tâches de manière simple et intuitive.

---

## Fonctionnalités

- **Ajouter une tâche** : Créez une nouvelle tâche avec un titre, une description et un statut.
- **Lister les tâches** : Affichez toutes les tâches existantes, y compris leur ID, leur titre, leur description et leur statut.
- **Modifier une tâche** : Changez les informations d'une tâche existante (titre, description ou statut).
- **Supprimer une tâche** : Retirez une tâche de la liste grâce à son ID.
- **Sauvegarde et chargement automatiques avec JSON** :
  - Les tâches sont sauvegardées dans un fichier JSON (`tasks.json`) lors de la fermeture de l'application.
  - Les tâches précédemment enregistrées sont automatiquement chargées au démarrage, si le fichier JSON existe.
- **Gestion des erreurs JSON** : Le programme gère les erreurs de format ou de lecture du fichier JSON et permet de le réinitialiser en cas de problème.

---

## Comment lancer le projet

### Prérequis

- **Rust** : Installez Rust via [rustup](https://rustup.rs/).

### Installation

1. Clonez le projet depuis ce dépôt :
   ```bash
   git clone <lien-du-dépôt>
   cd <dossier-du-projet>
   ```

2. Compilez le projet avec `cargo` :
   ```bash
   cargo build --release
   ```

### Exécution

Pour démarrer l'application, exécutez la commande suivante dans le terminal :
```bash
cargo run
```

---

## Structure du Projet

- **main.rs** : Gère le menu principal et les interactions utilisateur.
- **task_manager.rs** : Contient la logique principale de gestion des tâches (ajout, suppression, modification, etc.).
- **todo.rs** : Définition des structures `ToDo` et `Status` pour représenter les tâches.
- **utility.rs** : Gestion des fichiers JSON (création, chargement, sauvegarde).
- **input.rs** : Fonctions pour gérer les entrées utilisateur (texte, nombres, statut).

---

## Exemple d'utilisation

1. **Ajouter une tâche** :
   - Saisissez un titre, une description, et choisissez un statut (1 : À faire, 2 : En cours, 3 : Terminé).
2. **Lister les tâches** :
   - Visualisez toutes les tâches avec leur ID et leur statut.
3. **Modifier une tâche** :
   - Sélectionnez une tâche par son ID, puis modifiez son titre, sa description ou son statut.
4. **Sauvegarde automatique** :
   - Les modifications sont sauvegardées automatiquement dans `tasks.json` à la fermeture.

---

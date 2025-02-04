# TP1 : Tooling

## ✏️Auteurs✏️
- Tom DUNET (DUNT18030400)

## 📖Sujet📖

Développer un petit outil en ligne de commande capable de :
- Lire un fichier UPROJECT (2 pts)
  - Afficher le nom du jeu
  - Afficher la version de Unreal utilisée
    - Marquer « From Source » si cela est le cas
  - Afficher les plugins utilisés
- Compiler un projet Unreal (3 pts)
  - En utilisant UBT
- Packager un projet Unreal (5 pts)
  - En utilisant UAT

Je ne vous demande pas d’interface graphique. Néanmoins, j’offre jusqu’à 5 points bonus pour les GUI. Attention, une simple GUI ne su\ira pas, je jugerai l’expérience utilisateur de celle-ci tout comme son extension pour les futurs développements.


## 💻Utilisation💻

Veuillez suivre les étapes en fonction de votre système d'exploitation.

> **Note :** Tous les tests ont été effectués sous Windows, et aucune vérification n’a pu être réalisée sous macOS.

### Windows

1. **cloner le projet**

ouvrir un terminal et exécuter la commande suivante :
```bash
git clone https://github.com/Oridoshi/UnrealToolPerso
```

2. **Compiler le projet**

Une fois le projet cloné, naviguez dans le dossier `UnrealToolPerso` et exécutez la commande suivante :
```bash
cargo build --release
```

3. **Deplacer le .exe**

Une fois le build terminé, le fichier exécutable se trouve dans le dossier `target/release`.
Déplacez-le dans la racine de votre unreal from source.

4. **Lancer le programme**

Executez le fichier `UnrealToolPerso.exe` dans la racine de votre projet Unreal Engine.

5. **Utilisation du programme**

Le programme vous demandera de rentrer 2 chemins de fichiers, vous avez la possiblilité de les rentrer manuellement ou cliquer sur le bouton `Browse` pour le selectionner.

Si votre dossier contient un fichier `.uproject`, le programme affichera les informations de celui-ci et vous permettra de compiler votre projet.

Si vous avez rentré un chemin de fichier valide pour la sortie, alors le programme vous permettra de packager votre projet.

### MacOS

1. **cloner le projet**

ouvrir un terminal et exécuter la commande suivante :
```bash
git clone https://github.com/Oridoshi/UnrealToolPerso
```

2. **Compiler le projet**

Une fois le projet cloné, naviguez dans le dossier `UnrealToolPerso` et exécutez la commande suivante :
```bash
cargo build --release
```

3. **Deplacer le .app**

Une fois le build terminé, le fichier exécutable se trouve dans le dossier `target/release`.
Déplacez-le dans la racine de votre unreal from source.

4. **Lancer le programme**

Executez le fichier `UnrealToolPerso.app` dans la racine de votre projet Unreal Engine.

5. **Utilisation du programme**

Le programme vous demandera de rentrer 2 chemins de fichiers, vous avez la possiblilité de les rentrer manuellement ou cliquer sur le bouton `Browse` pour le selectionner.

Si votre dossier contient un fichier `.uproject`, le programme affichera les informations de celui-ci et vous permettra de compiler votre projet.

Si vous avez rentré un chemin de fichier valide pour la sortie, alors le programme vous permettra de packager votre projet.
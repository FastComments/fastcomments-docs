Maintenant que nous avons téléchargé le fichier zip, extrayez-le dans un dossier. J'ai téléchargé le fichier `casper.zip` par défaut et l'ai extrait dans `Downloads\casper` sous Windows.

Next, you'll want to make sure you have the LTS or newer version of NodeJS installed. You can get that here: https://nodejs.org/en/download/

Once NodeJS is installed, you'll want to install a code editor.

We recommend (and use) Webstorm, which you can get here with a 30-day trial (no credit card needed): https://www.jetbrains.com/webstorm/

The next best free option would probably be Visual Studio Code: https://code.visualstudio.com/download

Once you have your editor setup, and the theme folder open in the editor, open the terminal in the IDE and run:

[inline-code-attrs-start title = 'Installer le thème'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install
[inline-code-end]

La sortie réussie ressemblera à ceci (vous pouvez ignorer les avertissements) :

<div class="screenshot white-bg">
    <div class="title">Sortie réussie de npm install</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-2-1-install.png" alt="Sortie réussie de npm install" />
</div>

Cela configurera les dépendances du thème pour les commandes que nous exécuterons ultérieurement. De plus, l'exportation dépend de l'installation des dépendances du thème ; sinon, la réimportation ne fonctionnera pas correctement.
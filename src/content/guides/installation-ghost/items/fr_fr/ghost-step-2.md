Maintenant que nous avons téléchargé le fichier zip, extrayez-le dans un dossier. J'ai téléchargé le fichier `casper.zip` par défaut et je l'ai extrait dans `Downloads\casper` sous Windows.

Ensuite, vous devez vous assurer d'avoir installé la version LTS ou plus récente de NodeJS. Vous pouvez l'obtenir ici : https://nodejs.org/en/download/

Une fois NodeJS installé, vous devrez installer un éditeur de code.

Nous recommandons (et utilisons) Webstorm, que vous pouvez obtenir ici avec un essai de 30 jours (sans carte bancaire) : https://www.jetbrains.com/webstorm/

La meilleure option gratuite suivante serait probablement Visual Studio Code : https://code.visualstudio.com/download

Une fois votre éditeur configuré et le dossier du thème ouvert dans l'éditeur, ouvrez le terminal dans l'IDE et exécutez :

[inline-code-attrs-start title = 'Installer le thème'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install
[inline-code-end]

Une sortie réussie ressemblera à ceci (vous pouvez ignorer les avertissements) :

<div class="screenshot white-bg">
    <div class="title">Sortie réussie de npm install</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-2-1-install.png" alt="Sortie réussie de npm install" />
</div>

Cela configurera les dépendances du thème pour les commandes que nous exécuterons plus tard. De plus, l'exportation dépend de l'installation des dépendances du thème, sinon la réimportation ne fonctionnera pas correctement.
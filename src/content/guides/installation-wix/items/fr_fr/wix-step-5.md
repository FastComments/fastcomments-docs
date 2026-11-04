Ensuite, configurons les choses pour que le fil de commentaires change en fonction de la page actuelle, permettant aux utilisateurs de discuter du contenu affiché.

Sans les étapes suivantes, vous n'aurez qu'un seul fil de commentaires global pour tout votre site - ce qui n'est pas très utile.

#### Mode Dev

Pour ajouter cette fonctionnalité, nous devrons passer dans ce que Wix appelle le `Dev Mode`.

Cliquez sur l'option `Dev Mode` en haut de l'écran.

<div class="screenshot white-bg">
    <div class="title">Activer le Dev Mode</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-dev-mode.png" alt="Activer le Dev Mode" />
</div>

#### Définir l'ID de l'élément

Nous allons ajouter du code personnalisé pour accomplir cela, mais d'abord nous devons donner au nouvel élément intégré un ID pour pouvoir s'y référer.

Appelons-le `fastcomments`.

Cliquez sur le nouvel élément intégré que nous avons ajouté, et en mode dev en bas à droite vous devriez voir un champ ID avec une valeur comme `html1` :

<div class="screenshot white-bg">
    <div class="title">Le champ ID</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-0.png" alt="Le champ ID" />
</div>

Changez-le en `fastcomments` et appuyez sur entrée :

<div class="screenshot white-bg">
    <div class="title">Définir l'ID</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-1.png" alt="Définir l'ID" />
</div>

Maintenant nous pouvons ajouter notre code personnalisé qui indique à la zone de commentaires quelle page nous consultons.

En bas de l'écran vous devriez voir un éditeur de code comme celui-ci :

<div class="screenshot white-bg">
    <div class="title">Ouvrir l'éditeur</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-open-editor.png" alt="Définir l'ID" />
</div>

Copiez le code suivant et collez-le dedans :

[inline-code-attrs-start title = 'Extrait de navigation des commentaires Wix'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import wixLocation from 'wix-location';

$w.onReady(function () {
    function updateFastCommentsLocation() {
        try {
            const url = (wixLocation.baseUrl + '/' + wixLocation.path).replace(/,/g, '/');
            $w('#fastcomments').postMessage({'action': 'reload', 'url': url});
        } catch (err) {
            console.error('Wix -> FastComments Error', err);
        }
    }

    updateFastCommentsLocation();
    wixLocation.onChange( () => {
        updateFastCommentsLocation();
    });
});
[inline-code-end]

<div class="screenshot white-bg">
    <div class="title">Ajouter le code de navigation</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-paste-code.png" alt="Ajouter le code de navigation" />
</div>
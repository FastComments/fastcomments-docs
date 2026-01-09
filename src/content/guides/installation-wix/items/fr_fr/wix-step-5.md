Ensuite, configurons cela pour que le fil de commentaires change en fonction de la page actuelle, permettant aux utilisateurs de discuter du contenu affiché.

Sans les étapes suivantes, vous n'aurez qu'un seul fil de commentaires global pour l'ensemble du site - ce qui n'est pas très utile.

#### Dev Mode

Pour ajouter cette fonctionnalité, nous devons accéder à ce que Wix appelle `Dev Mode`.

Cliquez sur l'option `Dev Mode` en haut de l'écran.

<div class="screenshot white-bg">
    <div class="title">Activer Dev Mode</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-dev-mode.png" alt="Activer Dev Mode" />
</div>

#### Définir l'ID de l'élément

Nous allons ajouter du code personnalisé pour y parvenir, mais d'abord nous devons attribuer un ID à ce nouvel élément intégré afin d'y faire référence.

Appelons-le `fastcomments`.

Cliquez sur le nouvel élément intégré que vous avez ajouté, et en mode Dev, en bas à droite, vous devriez voir un champ ID avec une valeur comme `html1` :

<div class="screenshot white-bg">
    <div class="title">Le champ ID</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-0.png" alt="Le champ ID" />
</div>

Changez ceci en `fastcomments` et appuyez sur Entrée :

<div class="screenshot white-bg">
    <div class="title">Définir l'ID</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-1.png" alt="Définir l'ID" />
</div>

Nous pouvons maintenant ajouter notre code personnalisé qui indique à la zone de commentaires quelle page est affichée.

En bas de l'écran, vous devriez voir un éditeur de code comme ceci :

<div class="screenshot white-bg">
    <div class="title">Ouvrir l'éditeur</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-open-editor.png" alt="Ouvrir l'éditeur" />
</div>

Copiez le code suivant et collez-le dedans :

[inline-code-attrs-start title = 'Extrait de navigation pour Wix Comments'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
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

---
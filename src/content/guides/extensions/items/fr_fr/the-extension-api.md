Interagir avec l'`Extension` est simple, car nous définissons simplement des références aux fonctions que nous voulons invoquer.

Pour prolonger l'exemple précédent, disons que nous voulons ajouter du HTML en haut de chaque commentaire :

[inline-code-attrs-start title = 'Une extension simple - Suite'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
(function () {
    const extension = FastCommentsUI.extensions.find((extension) => {
        return extension.id === 'my-extension';
    });

    extension.commentFilter = function(comment, html) {
        return `<h3>The user's name is ${comment.commenterName}!</h3>` + html;
    }
})();
[inline-code-end]

Chaque fois que vous renvoyez du HTML de cette manière, il sera fusionné dans l'interface via un algorithme de diff du DOM.

#### Déclencher manuellement le re-rendu d'un commentaire

Nous pouvons attendre le chargement initial de la page et re-rendre manuellement un commentaire en appelant `reRenderComment` :

[inline-code-attrs-start title = "Re-rendu d'un commentaire"; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
(function () {
    const extension = FastCommentsUI.extensions.find((extension) => {
        return extension.id === 'my-extension';
    });

    let renderCount = 0;

    extension.commentFilter = function(comment, html) {
        renderCount++;
        return `<h3>The render count is ${renderCount}!</h3>` + html;
    }

    extension.onInitialRenderComplete = function() {
        setInterval(function() {
            extension.reRenderComment(extension.commentsById[Object.keys(extension.commentsById)[0]], function renderDone() {
                console.log('Comment re-render done.');
            });
        }, 2000); // timeout not required, just an example.
    }
})();
[inline-code-end]

---
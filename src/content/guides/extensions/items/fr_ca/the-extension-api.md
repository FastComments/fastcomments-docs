Interagir avec l'`Extension` est simple, car nous définissons simplement des références aux fonctions que nous souhaitons invoquer.

Pour reprendre l'exemple précédent, supposons que nous voulons ajouter du HTML au haut de chaque commentaire :

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

Chaque fois que vous renverrez du HTML comme ceci, il sera fusionné dans l'interface utilisateur via un algorithme de diff du DOM.

#### Déclencher manuellement le re-rendu d'un commentaire

Nous pouvons attendre le chargement initial de la page et relancer manuellement le rendu d'un commentaire en invoquant `reRenderComment`:

[inline-code-attrs-start title = 'Relancer le rendu d'un commentaire'; type = 'javascript'; inline-code-attrs-end]
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
        }, 2000); // délai non requis, juste un exemple.
    }
})();
[inline-code-end]

---
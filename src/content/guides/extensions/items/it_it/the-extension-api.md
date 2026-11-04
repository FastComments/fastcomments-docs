Interagire con l'`Extension` è semplice, poiché definiamo semplicemente riferimenti alle funzioni che vogliamo vengano invocate.

Per continuare l'esempio precedente, diciamo che vogliamo aggiungere HTML in cima a ogni commento:

[inline-code-attrs-start title = 'Una semplice estensione - Continua'; type = 'javascript'; inline-code-attrs-end]
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

Ogni volta che restituisci HTML come questo, verrà integrato nella UI tramite un algoritmo di dom-diffing.

#### Trigger manuale del re-render di un commento

Possiamo attendere il caricamento iniziale della pagina e rieseguire manualmente il render di un commento invocando `reRenderComment`:

[inline-code-attrs-start title = 'Re-Render di un Commento'; type = 'javascript'; inline-code-attrs-end]
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
        }, 2000); // timeout non richiesto, solo un esempio.
    }
})();
[inline-code-end]
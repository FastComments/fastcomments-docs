Interacting with the `Extension` is simple, as we simply define references to functions we want invoked.

Per basarci sull'esempio precedente, supponiamo di voler aggiungere HTML all'inizio di ogni commento:

[inline-code-attrs-start title = 'Una semplice estensione - Continuazione'; type = 'javascript'; inline-code-attrs-end]
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

Whenever you return HTML like this, it will get merged into the UI via a dom-diffing algorithm.

#### Attivare manualmente il re-render di un commento

Possiamo attendere il caricamento iniziale della pagina e renderizzare nuovamente un commento manualmente invocando `reRenderComment`:

[inline-code-attrs-start title = 'Re-Rendering di un commento'; type = 'javascript'; inline-code-attrs-end]
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
        }, 2000); // timeout non necessario, solo un esempio.
    }
})();
[inline-code-end]

---
Interactuar con la `Extension` es sencillo, ya que simplemente definimos referencias a las funciones que queremos invocar.

Para basarnos en el ejemplo anterior, digamos que queremos añadir HTML al principio de cada comentario:

[inline-code-attrs-start title = 'Una extensión simple - Continuación'; type = 'javascript'; inline-code-attrs-end]
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

Cada vez que devuelvas HTML así, se fusionará en la interfaz mediante un algoritmo de dom-diffing.

#### Volver a renderizar un comentario manualmente

Podemos esperar a la carga inicial de la página y volver a renderizar manualmente un comentario invocando `reRenderComment`:

[inline-code-attrs-start title = 'Volver a renderizar un comentario'; type = 'javascript'; inline-code-attrs-end]
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
        }, 2000); // el timeout no es necesario, solo un ejemplo.
    }
})();
[inline-code-end]

---
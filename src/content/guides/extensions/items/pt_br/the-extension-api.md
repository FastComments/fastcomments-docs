Interagir com a `Extension` é simples, pois apenas definimos referências para as funções que queremos que sejam invocadas.

Para continuar o exemplo anterior, digamos que queremos adicionar HTML ao topo de cada comentário:

[inline-code-attrs-start title = 'Uma Extensão Simples - Continuação'; type = 'javascript'; inline-code-attrs-end]
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

Sempre que você retornar HTML como este, ele será mesclado na UI por meio de um algoritmo de dom-diffing.

#### Acionando manualmente a re-renderização de um comentário

Podemos aguardar o carregamento inicial da página e re-renderizar manualmente um comentário invocando `reRenderComment`:

[inline-code-attrs-start title = 'Re-Renderizando um Comentário'; type = 'javascript'; inline-code-attrs-end]
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
        }, 2000); // o timeout não é obrigatório, é apenas um exemplo.
    }
})();
[inline-code-end]
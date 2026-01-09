Interakcja z `Extension` jest prosta — wystarczy zdefiniować odniesienia do funkcji, które chcemy wywołać.

Bazując na wcześniejszym przykładzie, załóżmy, że chcemy dodać HTML na początku każdego komentarza:

[inline-code-attrs-start title = 'Proste rozszerzenie - ciąg dalszy'; type = 'javascript'; inline-code-attrs-end]
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

Kiedykolwiek zwrócisz HTML w ten sposób, zostanie on scalony z interfejsem użytkownika za pomocą algorytmu porównywania DOM (dom-diffing).

#### Ręczne wywoływanie ponownego renderowania komentarza

Możemy poczekać na początkowe załadowanie strony i ręcznie ponownie wyrenderować komentarz, wywołując `reRenderComment`:

[inline-code-attrs-start title = 'Ponowne renderowanie komentarza'; type = 'javascript'; inline-code-attrs-end]
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
        }, 2000); // timeout nie jest wymagany, to tylko przykład.
    }
})();
[inline-code-end]

---
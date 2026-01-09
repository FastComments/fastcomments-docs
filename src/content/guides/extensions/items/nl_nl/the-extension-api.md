Interactie met de `Extension` is eenvoudig: we definiëren gewoon referenties naar functies die we willen aanroepen.

Om voort te bouwen op het eerdere voorbeeld, laten we zeggen dat we HTML aan de bovenkant van elke reactie willen toevoegen:

[inline-code-attrs-start title = 'Een eenvoudige extensie - Vervolg'; type = 'javascript'; inline-code-attrs-end]
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

Wanneer je HTML op deze manier terugstuurt, wordt deze samengevoegd in de UI via een dom-diffing-algoritme.

#### Het handmatig triggeren van het opnieuw renderen van een reactie

We kunnen wachten op de eerste paginalading en handmatig een reactie opnieuw renderen door `reRenderComment` aan te roepen:

[inline-code-attrs-start title = 'Opnieuw renderen van een reactie'; type = 'javascript'; inline-code-attrs-end]
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
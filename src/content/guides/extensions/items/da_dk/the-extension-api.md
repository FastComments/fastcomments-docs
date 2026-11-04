Interaktion med `Extension` er enkel, da vi blot definerer referencer til de funktioner, vi ønsker kaldt.

For at bygge videre på det tidligere eksempel, lad os sige, at vi vil tilføje HTML til toppen af hver kommentar:

[inline-code-attrs-start title = 'En simpel Extension - fortsat'; type = 'javascript'; inline-code-attrs-end]
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

Når du returnerer HTML som denne, vil den blive flettet ind i UI'en via en dom-diffing-algoritme.

#### Manuelt udløse re-rendering af en kommentar

Vi kan vente på den indledende sideindlæsning og manuelt re-rendere en kommentar ved at kalde `reRenderComment`:

[inline-code-attrs-start title = 'Re-rendering af en kommentar'; type = 'javascript'; inline-code-attrs-end]
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
        }, 2000); // timeout er ikke påkrævet, kun et eksempel.
    }
})();
[inline-code-end]
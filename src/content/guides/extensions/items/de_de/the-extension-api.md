Die Interaktion mit der `Extension` ist einfach, da wir lediglich Verweise auf Funktionen definieren, die aufgerufen werden sollen.

Um am vorherigen Beispiel anzuknüpfen, nehmen wir an, wir möchten HTML am Anfang jedes Kommentars hinzufügen:

[inline-code-attrs-start title = 'Eine einfache Erweiterung - Fortsetzung'; type = 'javascript'; inline-code-attrs-end]
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

Wann immer Sie auf diese Weise HTML zurückgeben, wird es mittels eines DOM-Diffing-Algorithmus in die UI integriert.

#### Manuelles Auslösen des erneuten Renderns eines Kommentars

Wir können auf das initiale Laden der Seite warten und einen Kommentar manuell neu rendern, indem wir `reRenderComment` aufrufen:

[inline-code-attrs-start title = 'Einen Kommentar neu rendern'; type = 'javascript'; inline-code-attrs-end]
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
        }, 2000); // Timeout nicht erforderlich, nur ein Beispiel.
    }
})();
[inline-code-end]

---
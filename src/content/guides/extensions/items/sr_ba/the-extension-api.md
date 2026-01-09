Interakcija sa `Extension` je jednostavna, jer jednostavno definišemo reference na funkcije koje želimo pozvati.

Da nadogradimo raniji primjer, recimo da želimo dodati HTML na vrh svakog komentara:

[inline-code-attrs-start title = 'Jednostavna ekstenzija - nastavak'; type = 'javascript'; inline-code-attrs-end]
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

Kad god vratite HTML ovako, on će biti spojen u UI putem DOM-diffing algoritma.

#### Ručno pokretanje ponovnog renderovanja komentara

Možemo sačekati inicijalno učitavanje stranice i ručno ponovo renderovati komentar pozivanjem `reRenderComment`:

[inline-code-attrs-start title = 'Ponovno renderovanje komentara'; type = 'javascript'; inline-code-attrs-end]
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
        }, 2000); // timeout nije potreban, samo primjer.
    }
})();
[inline-code-end]

---
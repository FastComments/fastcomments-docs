Interakcija z `Extension` je preprosta, saj le definiramo reference na funkcije, ki jih želimo izvesti.

Če nadaljujemo prejšnji primer, recimo, da želimo dodati HTML na vrh vsakega komentarja:

[inline-code-attrs-start title = 'Preprosta razširitev - nadaljevanje'; type = 'javascript'; inline-code-attrs-end]
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

Kadar koli vrnete HTML takole, bo vgrajen v uporabniški vmesnik prek algoritma za dom-diffing.

#### Ročno sprožitev ponovnega izrisa komentarja

Lahko počakamo na začetno nalaganje strani in ročno ponovno izrišemo komentar z uporabo `reRenderComment`:

[inline-code-attrs-start title = 'Ponovni izris komentarja'; type = 'javascript'; inline-code-attrs-end]
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
        }, 2000); // timeout ni obvezen, samo primer.
    }
})();
[inline-code-end]
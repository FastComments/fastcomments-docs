Interakcija са `Extension` је једноставна, јер једноставно дефинишемо референце на функције које желимо да се позову.

Да надовежемо на пример раније, рецимо да желимо да додамо HTML на врх сваког коментара:

[inline-code-attrs-start title = 'Једноставно проширење - наставак'; type = 'javascript'; inline-code-attrs-end]
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

Кад год вратите HTML на овај начин, он ће бити спојен у UI помоћу dom-diffing алгоритма.

#### Ручно покретање поновног рендеровања коментара

Можемо сачекати почетно учитавање странице и ручно поново рендеровати коментар позивањем `reRenderComment`:

[inline-code-attrs-start title = 'Поновно рендеровање коментара'; type = 'javascript'; inline-code-attrs-end]
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
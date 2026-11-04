Интеракција са `Extension` је једноставна, јер само дефинишемо референце на функције које желимо да буду позване.

Да наставимо са претходним примером, рецимо да желимо да додамо HTML на врх сваког коментара:

[inline-code-attrs-start title = 'Једноставан Extension - наставак'; type = 'javascript'; inline-code-attrs-end]
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

Кад год вратите HTML овако, он ће бити спојен у кориснички интерфејс помоћу алгоритма за dom-diffing.

#### Ручно покретање поновног приказивања коментара

Можемо сачекати почетно учитавање странице и ручно поново приказати коментар позивањем `reRenderComment`:

[inline-code-attrs-start title = 'Поновно приказивање коментара'; type = 'javascript'; inline-code-attrs-end]
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
        }, 2000); // timeout није обавезан, само пример.
    }
})();
[inline-code-end]
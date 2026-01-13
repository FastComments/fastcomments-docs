Взаимодействието с `Extension` е просто, тъй като просто дефинираме референции към функции, които искаме да бъдат извикани.

За да надградим предишния пример, да кажем, че искаме да добавим HTML в горната част на всеки коментар:

[inline-code-attrs-start title = 'Просто разширение - Продължение'; type = 'javascript'; inline-code-attrs-end]
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

Всеки път, когато върнете HTML по този начин, той ще бъде обединен в потребителския интерфейс чрез алгоритъм за dom-diffing.

#### Ръчно задействане на повторното изобразяване на коментар

Можем да изчакаме първоначалното зареждане на страницата и да ръчно преизобразим коментар, като извикаме `reRenderComment`:

[inline-code-attrs-start title = 'Повторно изобразяване на коментар'; type = 'javascript'; inline-code-attrs-end]
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
        }, 2000); // таймаутът не е необходим, просто пример.
    }
})();
[inline-code-end]

---
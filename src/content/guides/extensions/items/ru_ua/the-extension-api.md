Взаимодействовать с `Extension` просто, так как мы просто определяем ссылки на функции, которые хотим вызвать.

Чтобы продолжить предыдущий пример, допустим, мы хотим добавить HTML в начало каждого комментария:

[inline-code-attrs-start title = 'Простое расширение - продолжение'; type = 'javascript'; inline-code-attrs-end]
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

Всякий раз, когда вы возвращаете HTML подобным образом, он будет объединён с UI с помощью алгоритма сравнения DOM.

#### Ручной запуск повторного рендеринга комментария

Мы можем дождаться начальной загрузки страницы и вручную перерендерить комментарий, вызвав `reRenderComment`:

[inline-code-attrs-start title = 'Повторный рендер комментария'; type = 'javascript'; inline-code-attrs-end]
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
        }, 2000); // таймаут не обязателен, это просто пример.
    }
})();
[inline-code-end]

---
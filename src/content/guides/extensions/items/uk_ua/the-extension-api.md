Взаємодія з `Extension` проста: ми просто визначаємо посилання на функції, які хочемо викликати.

Продовжуючи попередній приклад, припустімо, що ми хочемо додати HTML зверху кожного коментаря:

[inline-code-attrs-start title = 'Просте розширення — продовження'; type = 'javascript'; inline-code-attrs-end]
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

Коли ви повертаєте HTML таким чином, він буде об'єднаний з UI за допомогою алгоритму dom-diffing.

#### Ручний повторний рендер коментаря

Ми можемо дочекатися початкового завантаження сторінки і вручну повторно відрендерити коментар, викликавши `reRenderComment`:

[inline-code-attrs-start title = 'Повторний рендер коментаря'; type = 'javascript'; inline-code-attrs-end]
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
        }, 2000); // таймаут не обов'язковий, просто приклад.
    }
})();
[inline-code-end]

---
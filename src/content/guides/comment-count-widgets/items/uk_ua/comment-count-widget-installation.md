Віджет підрахунку коментарів призначений для відображення кількості коментарів окремої сторінки. Він легкий і забезпечує оновлення в реальному часі, якщо налаштований.

## Базове встановлення

[inline-code-attrs-start title = 'Comment Count Widget Installation'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-comment-count.min.js"></script>
<div id="comment-count"></div>
<script>
    window.FastCommentsCommentCount(document.getElementById('comment-count'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

## Параметри конфігурації

Функція `FastCommentsCommentCount` приймає наступні параметри конфігурації:

- **tenantId** (обов'язково): Ваш ідентифікатор орендаря FastComments
- **urlId** (необов'язково): Ідентифікатор сторінки. За замовчуванням `window.location.href`, якщо не вказано
- **numberOnly** (необов'язково): Якщо `true`, відображає лише число без тексту. За замовчуванням `false`
- **isLive** (необов'язково): Якщо `true`, лічильник буде оновлюватися автоматично. За замовчуванням `false`

## Розширені приклади

### Користувацький URL ID

[inline-code-attrs-start title = 'Comment Count with Custom URL ID'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-comment-count.min.js"></script>
<div id="comment-count-custom"></div>
<script>
    window.FastCommentsCommentCount(document.getElementById('comment-count-custom'), {
        tenantId: 'demo',
        urlId: 'my-custom-page-id'
    });
</script>
[inline-code-end]

### Відображення лише числа

[inline-code-attrs-start title = 'Comment Count Number Only'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-comment-count.min.js"></script>
<div id="comment-count-number"></div>
<script>
    window.FastCommentsCommentCount(document.getElementById('comment-count-number'), {
        tenantId: 'demo',
        numberOnly: true
    });
</script>
[inline-code-end]

### Оновлення в реальному часі

[inline-code-attrs-start title = 'Live Comment Count Updates'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-comment-count.min.js"></script>
<div id="comment-count-live"></div>
<script>
    window.FastCommentsCommentCount(document.getElementById('comment-count-live'), {
        tenantId: 'demo',
        isLive: true
    });
</script>
[inline-code-end]

## Методи віджета

Віджет повертає об'єкт з наступними методами:

- **destroy()**: Видаляє віджет і очищає всі таймери
- **update(config)**: Оновлює віджет з новою конфігурацією

### Приклад використання

[inline-code-attrs-start title = 'Widget Methods Example'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-comment-count.min.js"></script>
<div id="comment-count-methods"></div>
<script>
    const widget = window.FastCommentsCommentCount(document.getElementById('comment-count-methods'), {
        tenantId: 'demo'
    });

    // Update the widget to show a different page's count
    setTimeout(() => {
        widget.update({
            tenantId: 'demo',
            urlId: 'different-page-id'
        });
    }, 5000);

    // Destroy the widget after 10 seconds
    setTimeout(() => {
        widget.destroy();
    }, 10000);
</script>
[inline-code-end]

## Стилізація

Віджет рендерить простий HTML з кількістю коментарів і постачається з мінімальною стилізацією. Ви можете налаштувати зовнішній вигляд за допомогою CSS:

[inline-code-attrs-start title = 'Custom Styling'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<style>
    .comment-count-styled {
        background: #f0f0f0;
        padding: 5px 10px;
        border-radius: 15px;
        font-size: 14px;
        color: #666;
        display: inline-block;
    }
</style>
<script src="https://cdn.fastcomments.com/js/widget-comment-count.min.js"></script>
<div id="comment-count-styled" class="comment-count-styled"></div>
<script>
    window.FastCommentsCommentCount(document.getElementById('comment-count-styled'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

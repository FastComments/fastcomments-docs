Виджет подсчета комментариев предназначен для отображения количества комментариев на отдельной странице. Он легкий и обеспечивает обновления в реальном времени, если настроен.

## Базовая установка

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

## Параметры конфигурации

Функция `FastCommentsCommentCount` принимает следующие параметры конфигурации:

- **tenantId** (обязательно): Ваш идентификатор арендатора FastComments
- **urlId** (необязательно): Идентификатор страницы. По умолчанию `window.location.href`, если не указано
- **numberOnly** (необязательно): Если `true`, отображает только число без текста. По умолчанию `false`
- **isLive** (необязательно): Если `true`, счетчик будет обновляться автоматически. По умолчанию `false`

## Расширенные примеры

### Пользовательский URL ID

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

### Отображение только числа

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

### Обновления в реальном времени

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

## Методы виджета

Виджет возвращает объект со следующими методами:

- **destroy()**: Удаляет виджет и очищает все таймеры
- **update(config)**: Обновляет виджет с новой конфигурацией

### Пример использования

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

## Стилизация

Виджет рендерит простой HTML с количеством комментариев и поставляется с минимальной стилизацией. Вы можете настроить внешний вид с помощью CSS:

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

### Базовый пример

Самый простой способ использовать Image Chat — нацелиться на один элемент изображения. Этот пример показывает, как включить интерактивные обсуждения на изображении:

[inline-code-attrs-start title = 'Базовый пример Image Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>Product Image with Chat</title>
</head>
<body>
    <img id="product-image" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Product Photo" />

    <script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
    <script>
        FastCommentsImageChat(document.getElementById('product-image'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

### Пример с контейнером

Вы также можете передать элемент-контейнер, внутри которого находится изображение:

[inline-code-attrs-start title = 'Image Chat с контейнером'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<div id="image-container">
    <img src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="System Diagram" />
</div>

<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('image-container'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

### Пример с пользовательским URL ID

По умолчанию Image Chat использует URL страницы в сочетании с источником изображения и координатами для идентификации бесед. Вы можете указать пользовательский `urlId`:

[inline-code-attrs-start title = 'Image Chat с пользовательским URL ID'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('product-image'), {
        tenantId: 'demo',
        urlId: 'product-v2-main-image'
    });
</script>
[inline-code-end]

Это полезно, если структура ваших URL изменяется, но вы хотите сохранить те же беседы, или если вы хотите распространять одни и те же точки обсуждения на нескольких страницах.

### Пример с тёмным режимом

Если на вашем сайте тёмный фон и виджет не определяет это автоматически, как должен, мы можем вручную включить поддержку тёмного режима:

[inline-code-attrs-start title = 'Image Chat в тёмном режиме'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('product-image'), {
        tenantId: 'demo',
        hasDarkBackground: true
    });
</script>
[inline-code-end]

### Пример с пользовательским размером квадрата чата

Вы можете настроить размер кликабельных квадратиков, которые появляются на изображении. Размер задаётся в процентах от ширины изображения:

[inline-code-attrs-start title = 'Image Chat с пользовательским размером квадрата'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>Image Chat with Custom Square Size</title>
</head>
<body>
    <img id="product-image" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Product Photo" />

    <script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
    <script>
        FastCommentsImageChat(document.getElementById('product-image'), {
            tenantId: 'demo',
            chatSquarePercentage: 2, // Меньшие квадраты (по умолчанию 5)
        });
    </script>
</body>
</html>
[inline-code-end]

### Пример с обратным вызовом количества комментариев

Отслеживайте добавление или обновление комментариев с помощью обратного вызова `commentCountUpdated`:

[inline-code-attrs-start title = 'Image Chat с обратным вызовом при обновлении количества комментариев'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('product-image'), {
        tenantId: 'demo',
        commentCountUpdated: function(count) {
            console.log('Total comments:', count);
            document.getElementById('comment-badge').textContent = count;
        }
    });
</script>
[inline-code-end]

### Пример с несколькими изображениями

Вы можете инициализировать Image Chat на нескольких изображениях. У каждого изображения будут свои независимые точки обсуждения:

[inline-code-attrs-start title = 'Image Chat на нескольких изображениях'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<img id="image-1" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Photo 1" />
<img id="image-2" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Photo 2" />

<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    // Инициализация для первого изображения
    FastCommentsImageChat(document.getElementById('image-1'), {
        tenantId: 'demo',
        urlId: 'gallery-photo-1'
    });

    // Инициализация для второго изображения
    FastCommentsImageChat(document.getElementById('image-2'), {
        tenantId: 'demo',
        urlId: 'gallery-photo-2'
    });
</script>
[inline-code-end]

---
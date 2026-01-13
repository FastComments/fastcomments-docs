### Основен пример

Най-простият начин да използвате Image Chat е да насочите към един елемент изображение. Този пример показва как да активирате интерактивни дискусии върху изображение:

[inline-code-attrs-start title = 'Основен пример за Image Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

### Пример с контейнерен елемент

Можете също да предадете контейнерен елемент, който съдържа изображение вътре в него:

[inline-code-attrs-start title = 'Image Chat с контейнерен елемент'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

### Пример с персонализиран URL ID

По подразбиране Image Chat използва URL на страницата в комбинация с източника на изображението и координатите, за да идентифицира разговорите. Можете да зададете персонализиран `urlId`:

[inline-code-attrs-start title = 'Image Chat с потребителски URL ID'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('product-image'), {
        tenantId: 'demo',
        urlId: 'product-v2-main-image'
    });
</script>
[inline-code-end]

Това е полезно, ако структурата на вашите URL адреси се променя, но искате да запазите същите разговори, или ако искате да споделите едни и същи точки за обсъждане между няколко страници.

### Пример с тъмен режим

Ако вашият сайт има тъмен фон и джаджата не го открива автоматично, както би трябвало, можем ръчно да активираме поддръжка за тъмен режим:

[inline-code-attrs-start title = 'Image Chat с тъмен режим'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('product-image'), {
        tenantId: 'demo',
        hasDarkBackground: true
    });
</script>
[inline-code-end]

### Пример с персонализиран размер на чат-квадрата

Можете да регулирате размера на кликващите квадрати, които се появяват върху изображението. Размерът се задава като процент от ширината на изображението:

[inline-code-attrs-start title = 'Image Chat с персонализиран размер на квадратите'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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
            chatSquarePercentage: 2, // По-малки квадратчета (по подразбиране е 5)
        });
    </script>
</body>
</html>
[inline-code-end]

### Пример с callback за брой коментари

Проследявайте кога коментарите са добавени или обновени, използвайки `commentCountUpdated` callback:

[inline-code-attrs-start title = 'Image Chat с callback за брой коментари'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

### Пример с няколко изображения

Можете да инициализирате Image Chat на няколко изображения. Всяко изображение ще има свои собствени независими точки за дискусия:

[inline-code-attrs-start title = 'Image Chat върху няколко изображения'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<img id="image-1" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Photo 1" />
<img id="image-2" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Photo 2" />

<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    // Инициализиране за първото изображение
    FastCommentsImageChat(document.getElementById('image-1'), {
        tenantId: 'demo',
        urlId: 'gallery-photo-1'
    });

    // Инициализиране за второто изображение
    FastCommentsImageChat(document.getElementById('image-2'), {
        tenantId: 'demo',
        urlId: 'gallery-photo-2'
    });
</script>
[inline-code-end]

---
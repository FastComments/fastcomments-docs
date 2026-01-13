### Базовий приклад

Найпростіший спосіб використання Image Chat — застосувати його до одного елемента зображення. У цьому прикладі показано, як увімкнути інтерактивні обговорення на зображенні:

[inline-code-attrs-start title = 'Базовий приклад Image Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

### Приклад з контейнером

Ви також можете передати елемент-контейнер, всередині якого знаходиться зображення:

[inline-code-attrs-start title = 'Image Chat з контейнером'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

### Приклад з власним URL ID

За замовчуванням Image Chat використовує URL сторінки в поєднанні з джерелом зображення та координатами для ідентифікації розмов. Ви можете вказати власний `urlId`:

[inline-code-attrs-start title = 'Image Chat з власним URL ID'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('product-image'), {
        tenantId: 'demo',
        urlId: 'product-v2-main-image'
    });
</script>
[inline-code-end]

Це корисно, якщо структура ваших URL змінюється, але ви хочете зберегти ті самі розмови, або якщо ви хочете поширювати ті ж точки обговорення на кількох сторінках.

### Приклад з темним режимом

Якщо на вашому сайті темний фон і віджет не визначає це автоматично, як має, ми можемо вручну увімкнути підтримку темного режиму:

[inline-code-attrs-start title = 'Image Chat у темному режимі'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('product-image'), {
        tenantId: 'demo',
        hasDarkBackground: true
    });
</script>
[inline-code-end]

### Приклад з індивідуальним розміром квадратиків

Ви можете налаштувати розмір клікабельних квадратиків, що з'являються на зображенні. Розмір задається у відсотках від ширини зображення:

[inline-code-attrs-start title = 'Image Chat з індивідуальним розміром квадратиків'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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
            chatSquarePercentage: 2, // Менші квадрати (за замовчуванням 5)
        });
    </script>
</body>
</html>
[inline-code-end]

### Приклад зі зворотним викликом кількості коментарів

Відстежуйте додавання або оновлення коментарів за допомогою зворотного виклику `commentCountUpdated`:

[inline-code-attrs-start title = 'Image Chat зі зворотним викликом кількості коментарів'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

### Приклад для кількох зображень

Ви можете ініціалізувати Image Chat для кількох зображень. Кожне зображення матиме свої незалежні точки обговорення:

[inline-code-attrs-start title = 'Image Chat на кількох зображеннях'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<img id="image-1" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Photo 1" />
<img id="image-2" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Photo 2" />

<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    // Ініціалізація для першого зображення
    FastCommentsImageChat(document.getElementById('image-1'), {
        tenantId: 'demo',
        urlId: 'gallery-photo-1'
    });

    // Ініціалізація для другого зображення
    FastCommentsImageChat(document.getElementById('image-2'), {
        tenantId: 'demo',
        urlId: 'gallery-photo-2'
    });
</script>
[inline-code-end]

---
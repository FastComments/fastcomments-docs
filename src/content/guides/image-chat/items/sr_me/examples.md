### Основни пример

Најједноставнији начин коришћења Image Chat-а је да циљате један елемент слике. Овај пример показује како омогућити интерактивне дискусије на слици:

[inline-code-attrs-start title = 'Основни пример Image Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

### Пример са контејнер елементом

Такође можете проследити контејнер елемент који у себи има слику:

[inline-code-attrs-start title = 'Image Chat са контејнер елементом'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

### Пример са прилагођеним URL ID-јем

Подразумевано, Image Chat користи URL странице у комбинацији са извором слике и координатама да идентификује разговоре. Можете обезбиједити прилагођени `urlId`:

[inline-code-attrs-start title = 'Image Chat са прилагођеним URL ID-ом'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('product-image'), {
        tenantId: 'demo',
        urlId: 'product-v2-main-image'
    });
</script>
[inline-code-end]

Ово је корисно ако се структура ваших URL-ова промијени, али желите да очувате исте разговоре, или ако желите да дијелите иста мјеста за дискусију на више страница.

### Пример са тамним режимом

Ако ваша страница има тамну позадину и видгет је не детектује аутоматски као што би требало, можемо ручно омогућити подршку за тамни режим:

[inline-code-attrs-start title = 'Image Chat са тамним режимом'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('product-image'), {
        tenantId: 'demo',
        hasDarkBackground: true
    });
</script>
[inline-code-end]

### Пример са прилагођеном величином квадрата

Можете прилагодити величину кликабилних квадрата који се појављују на слици. Величина се одређује као проценат ширине слике:

[inline-code-attrs-start title = 'Image Chat са прилагођеном величином квадрата'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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
            chatSquarePercentage: 2, // Мањи квадрати (подразумјевано је 5)
        });
    </script>
</body>
</html>
[inline-code-end]

### Пример са повратним позивом за број коментара

Пратите када су коментари додати или ажурирани користећи повратни позив `commentCountUpdated`:

[inline-code-attrs-start title = 'Image Chat са повратним позивом за број коментара'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

### Пример са више слика

Можете иницијализовати Image Chat на више слика. Свака слика ће имати своје независне тачке за дискусију:

[inline-code-attrs-start title = 'Image Chat на више слика'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<img id="image-1" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Photo 1" />
<img id="image-2" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Photo 2" />

<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    // Иницијализујте за прву слику
    FastCommentsImageChat(document.getElementById('image-1'), {
        tenantId: 'demo',
        urlId: 'gallery-photo-1'
    });

    // Иницијализујте за другу слику
    FastCommentsImageChat(document.getElementById('image-2'), {
        tenantId: 'demo',
        urlId: 'gallery-photo-2'
    });
</script>
[inline-code-end]

---
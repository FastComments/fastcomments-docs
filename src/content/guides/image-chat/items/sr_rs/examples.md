### Основни пример

Најједноставнији начин за коришћење Image Chat‑а је да се циља један елемент слике. Овај пример показује како омогућити интерактивне дискусије на слици:

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

Можете такође проследити контејнер елемент који садржи слику унутар себе:

[inline-code-attrs-start title = 'Image Chat са контејнером'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

### Пример са прилагођеним URL ID

Подразумевано, Image Chat користи URL странице у комбинацији са извором слике и координатама за идентификацију разговора. Можете обезбедити прилагођени `urlId`:

[inline-code-attrs-start title = 'Image Chat са прилагођеним URL ID'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('product-image'), {
        tenantId: 'demo,
        urlId: 'product-v2-main-image'
    });
</script>
[inline-code-end]

Ово је корисно ако се ваша структура URL‑а промени, али желите да задржите исте разговоре, или ако желите да делите исте тачке дискусије на више страница.

### Пример са тамним режимом

Ако ваш сајт има тамну позадину и виџет не открива то аутоматски као што треба, можемо ручно омогућити подршку за тамни режим:

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

### Пример са прилагођеном величином квадрата за ћаскање

Можете подесити величину кликљивих квадрата који се појављују на слици. Величина се задаје као проценат ширине слике:

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
            chatSquarePercentage: 2, // Мањи квадрати (подразумевано је 5)
        });
    </script>
</body>
</html>
[inline-code-end]

### Пример са повратним позивом за број коментара

Пратите када се коментари додају или ажурирају користећи `commentCountUpdated` повратни позив:

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

Можете иницијализовати Image Chat на више слика. Свака слика ће имати своје независне тачке дискусије:

[inline-code-attrs-start title = 'Image Chat на више слика'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<img id="image-1" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Photo 1" />
<img id="image-2" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Photo 2" />

<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    // Initialize on first image
    FastCommentsImageChat(document.getElementById('image-1'), {
        tenantId: 'demo',
        urlId: 'gallery-photo-1'
    });

    // Initialize on second image
    FastCommentsImageChat(document.getElementById('image-2'), {
        tenantId: 'demo',
        urlId: 'gallery-photo-2'
    });
</script>
[inline-code-end]
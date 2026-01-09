### Основни пример

Најједноставнији начин кориштења Image Chat-а је да се усмјерите на један елемент слике. Овај примјер показује како омогућити интерактивне дискусије на слици:

[inline-code-attrs-start title = 'Основни пример Image Chat-а'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

### Пример са елементом контејнера

Можете такође прослиједити елемент контејнера који у себи садржи слику:

[inline-code-attrs-start title = 'Image Chat са елементом контејнера'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

### Пример са прилагођеним URL ID-ом

По подразумеваној поставци, Image Chat користи URL странице у комбинацији са извором слике и координатама да идентификује разговоре. Можете обезбиједити прилагођени `urlId`:

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

Ово је корисно ако се структура ваших URL-ова мијења, а желите да задржите исте разговоре, или ако желите дијелити исте тачке дискусије на више страница.

### Пример са тамним режимом

Ако ваша страница има тамну позадину и виџет је не детектује аутоматски како треба, можемо ручно омогућити подршку за тамни режим:

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

### Пример са прилагођеном величином квадрата за чат

Можете подесити величину кликабельних квадрата који се појављују на слици. Величина је назначена као проценат ширине слике:

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
            chatSquarePercentage: 2, // Мањи квадратићи (подразумевано је 5)
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

### Пример на више слика

Можете иницијализовати Image Chat на више слика. Свака слика ће имати своје независне тачке дискусије:

[inline-code-attrs-start title = 'Image Chat на више слика'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<img id="image-1" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Photo 1" />
<img id="image-2" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Photo 2" />

<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    // Иницијализујте на првој слици
    FastCommentsImageChat(document.getElementById('image-1'), {
        tenantId: 'demo',
        urlId: 'gallery-photo-1'
    });

    // Иницијализујте на другој слици
    FastCommentsImageChat(document.getElementById('image-2'), {
        tenantId: 'demo',
        urlId: 'gallery-photo-2'
    });
</script>
[inline-code-end]

---
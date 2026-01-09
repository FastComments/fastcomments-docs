### Grundlæggende eksempel

Den enkleste måde at bruge Image Chat på er at målrette et enkelt billedelement. Dette eksempel viser, hvordan man aktiverer interaktive diskussioner på et billede:

[inline-code-attrs-start title = 'Grundlæggende Image Chat-eksempel'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

### Eksempel med container-element

Du kan også give et container-element, der har et billede indeni:

[inline-code-attrs-start title = 'Image Chat with Container'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

### Eksempel med brugerdefineret URL-id

Som standard bruger Image Chat side-URL'en kombineret med billedets kilde og koordinater til at identificere samtaler. Du kan angive et brugerdefineret `urlId`:

[inline-code-attrs-start title = 'Image Chat with Custom URL ID'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('product-image'), {
        tenantId: 'demo',
        urlId: 'product-v2-main-image'
    });
</script>
[inline-code-end]

Det er nyttigt, hvis din URL-struktur ændrer sig, men du ønsker at bevare de samme samtaler, eller hvis du vil dele de samme diskussionspunkter på tværs af flere sider.

### Eksempel med mørk tilstand

Hvis dit site har en mørk baggrund, og widget'en ikke automatisk registrerer det som den burde, kan vi manuelt aktivere support for mørk tilstand:

[inline-code-attrs-start title = 'Image Chat with Dark Mode'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('product-image'), {
        tenantId: 'demo',
        hasDarkBackground: true
    });
</script>
[inline-code-end]

### Eksempel med brugerdefineret firkantstørrelse

Du kan justere størrelsen på de klikbare firkanter, der vises på billedet. Størrelsen angives som en procentdel af billedets bredde:

[inline-code-attrs-start title = 'Image Chat with Custom Square Size'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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
            chatSquarePercentage: 2, // Mindre firkanter (standard er 5)
        });
    </script>
</body>
</html>
[inline-code-end]

### Eksempel med kommentarantal-callback

Hold styr på, hvornår kommentarer tilføjes eller opdateres ved hjælp af `commentCountUpdated`-callback'en:

[inline-code-attrs-start title = 'Image Chat with Comment Count Callback'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

### Eksempel med flere billeder

Du kan initialisere Image Chat på flere billeder. Hvert billede får sine egne uafhængige diskussionspunkter:

[inline-code-attrs-start title = 'Image Chat on Multiple Images'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<img id="image-1" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Photo 1" />
<img id="image-2" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Photo 2" />

<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    // Initialiser på første billede
    FastCommentsImageChat(document.getElementById('image-1'), {
        tenantId: 'demo',
        urlId: 'gallery-photo-1'
    });

    // Initialiser på andet billede
    FastCommentsImageChat(document.getElementById('image-2'), {
        tenantId: 'demo',
        urlId: 'gallery-photo-2'
    });
</script>
[inline-code-end]

---
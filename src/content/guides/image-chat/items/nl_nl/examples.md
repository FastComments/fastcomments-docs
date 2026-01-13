### Basisvoorbeeld

De eenvoudigste manier om Image Chat te gebruiken is door één afbeeldingelement te selecteren. Dit voorbeeld toont hoe je interactieve discussies op een afbeelding inschakelt:

[inline-code-attrs-start title = 'Basisvoorbeeld Image Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

### Voorbeeld met container-element

Je kunt ook een containerelement doorgeven dat een afbeelding bevat:

[inline-code-attrs-start title = 'Image Chat met containerelement'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

### Voorbeeld met aangepaste URL-ID

Standaard gebruikt Image Chat de pagina-URL gecombineerd met de afbeeldingsbron en coördinaten om gesprekken te identificeren. Je kunt een aangepaste `urlId` opgeven:

[inline-code-attrs-start title = 'Image Chat met aangepaste URL-ID'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('product-image'), {
        tenantId: 'demo',
        urlId: 'product-v2-main-image'
    });
</script>
[inline-code-end]

Dit is handig als je URL-structuur verandert maar je dezelfde gesprekken wilt behouden, of als je dezelfde discussiepunten over meerdere pagina's wilt delen.

### Voorbeeld met donkere modus

Als je site een donkere achtergrond heeft en de widget deze niet automatisch detecteert zoals zou moeten, kun je handmatig ondersteuning voor de donkere modus inschakelen:

[inline-code-attrs-start title = 'Image Chat met donkere modus'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('product-image'), {
        tenantId: 'demo',
        hasDarkBackground: true
    });
</script>
[inline-code-end]

### Voorbeeld met aangepaste vierkantsgrootte

Je kunt de grootte aanpassen van de klikbare vierkantjes die op de afbeelding verschijnen. De grootte wordt opgegeven als een percentage van de afbeeldingsbreedte:

[inline-code-attrs-start title = 'Image Chat met aangepaste vierkantsgrootte'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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
            chatSquarePercentage: 2, // Kleinere vierkantjes (standaard is 5)
        });
    </script>
</body>
</html>
[inline-code-end]

### Voorbeeld met callback voor aantal reacties

Houd bij wanneer reacties worden toegevoegd of bijgewerkt met de `commentCountUpdated` callback:

[inline-code-attrs-start title = 'Image Chat met callback voor aantal reacties'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

### Voorbeeld met meerdere afbeeldingen

Je kunt Image Chat op meerdere afbeeldingen initialiseren. Elke afbeelding krijgt zijn eigen onafhankelijke discussiepunten:

[inline-code-attrs-start title = 'Image Chat op meerdere afbeeldingen'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<img id="image-1" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Photo 1" />
<img id="image-2" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Photo 2" />

<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    // Initialiseer op eerste afbeelding
    FastCommentsImageChat(document.getElementById('image-1'), {
        tenantId: 'demo',
        urlId: 'gallery-photo-1'
    });

    // Initialiseer op tweede afbeelding
    FastCommentsImageChat(document.getElementById('image-2'), {
        tenantId: 'demo',
        urlId: 'gallery-photo-2'
    });
</script>
[inline-code-end]

---
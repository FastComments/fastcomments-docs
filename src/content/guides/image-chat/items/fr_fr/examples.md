### Exemple de base

La façon la plus simple d'utiliser Image Chat est de cibler un seul élément image. Cet exemple montre comment activer des discussions interactives sur une image :

[inline-code-attrs-start title = 'Exemple basique d\'Image Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

### Exemple avec un élément conteneur

Vous pouvez également passer un élément conteneur qui contient une image à l'intérieur :

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

### Exemple avec un ID d'URL personnalisé

Par défaut, Image Chat utilise l'URL de la page combinée avec la source de l'image et les coordonnées pour identifier les conversations. Vous pouvez fournir un `urlId` personnalisé :

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

Ceci est utile si la structure de vos URL change mais que vous souhaitez conserver les mêmes conversations, ou si vous voulez partager les mêmes points de discussion sur plusieurs pages.

### Exemple avec le mode sombre

Si votre site a un arrière-plan sombre et que le widget ne le détecte pas automatiquement comme il le devrait, nous pouvons activer manuellement la prise en charge du mode sombre :

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

### Exemple avec une taille de carré de chat personnalisée

Vous pouvez ajuster la taille des carrés cliquables qui apparaissent sur l'image. La taille est spécifiée en pourcentage de la largeur de l'image :

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
            chatSquarePercentage: 2, // Carrés plus petits (la valeur par défaut est 5)
        });
    </script>
</body>
</html>
[inline-code-end]

### Exemple avec rappel du compteur de commentaires

Suivez lorsque des commentaires sont ajoutés ou mis à jour en utilisant le callback `commentCountUpdated` :

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

### Exemple avec plusieurs images

Vous pouvez initialiser Image Chat sur plusieurs images. Chaque image aura ses propres points de discussion indépendants :

[inline-code-attrs-start title = 'Image Chat on Multiple Images'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<img id="image-1" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Photo 1" />
<img id="image-2" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Photo 2" />

<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    // Initialiser sur la première image
    FastCommentsImageChat(document.getElementById('image-1'), {
        tenantId: 'demo',
        urlId: 'gallery-photo-1'
    });

    // Initialiser sur la deuxième image
    FastCommentsImageChat(document.getElementById('image-2'), {
        tenantId: 'demo',
        urlId: 'gallery-photo-2'
    });
</script>
[inline-code-end]

---
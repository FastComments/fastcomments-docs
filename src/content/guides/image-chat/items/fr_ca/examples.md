### Exemple de base

La façon la plus simple d'utiliser Image Chat est de cibler un seul élément image. Cet exemple montre comment activer des discussions interactives sur une image :

[inline-code-attrs-start title = 'Exemple de base pour Image Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

Vous pouvez également passer un élément conteneur qui contient une image :

[inline-code-attrs-start title = 'Image Chat avec un conteneur'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

### Exemple avec un ID URL personnalisé

Par défaut, Image Chat utilise l'URL de la page combinée avec la source de l'image et les coordonnées pour identifier les conversations. Vous pouvez fournir un `urlId` personnalisé :

[inline-code-attrs-start title = 'Image Chat avec un ID URL personnalisé'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

Si votre site a un fond sombre et que le widget ne le détecte pas automatiquement comme il le devrait, nous pouvons activer manuellement la prise en charge du mode sombre :

[inline-code-attrs-start title = 'Image Chat avec le mode sombre'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('product-image'), {
        tenantId: 'demo',
        hasDarkBackground: true
    });
</script>
[inline-code-end]

### Exemple avec une taille personnalisée des carrés de chat

Vous pouvez ajuster la taille des carrés cliquables qui apparaissent sur l'image. La taille est spécifiée en pourcentage de la largeur de l'image :

[inline-code-attrs-start title = 'Image Chat avec une taille personnalisée des carrés cliquables'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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
            chatSquarePercentage: 2, // Carrés plus petits (par défaut : 5)
        });
    </script>
</body>
</html>
[inline-code-end]

### Exemple avec une fonction de rappel du nombre de commentaires

Suivez quand des commentaires sont ajoutés ou mis à jour en utilisant le rappel `commentCountUpdated` :

[inline-code-attrs-start title = 'Image Chat avec une fonction de rappel du nombre de commentaires'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Image Chat sur plusieurs images'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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
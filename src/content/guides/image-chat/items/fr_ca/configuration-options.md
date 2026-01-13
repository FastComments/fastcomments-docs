### Overview

FastComments Image Chat étend le widget de commentaires FastComments standard, donc il hérite de toutes les options de configuration du widget de base tout en ajoutant quelques options spécifiques aux annotations d'images.

### Required Configuration

#### tenantId

Votre ID de locataire FastComments est requis. Vous pouvez le trouver dans votre [FastComments dashboard](https://fastcomments.com/auth/my-account/api-secret).

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo'
});
```

### Image Chat Specific Options

#### urlId

Par défaut, Image Chat génère un identifiant unique pour chaque conversation basé sur l'URL de la page, la source de l'image et les coordonnées X/Y. Vous pouvez remplacer ceci par un `urlId` personnalisé.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    urlId: 'my-custom-image-id'
});
```

Ceci est utile lorsque la structure de votre URL peut changer mais que vous souhaitez conserver les mêmes conversations, ou lorsque vous voulez partager des annotations entre plusieurs pages.

#### chatSquarePercentage

Contrôle la taille des marqueurs cliquables de chat en pourcentage de la largeur de l'image. La valeur par défaut est de 5 %, ce qui signifie que chaque marqueur représente 5 % de la largeur de l'image.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    chatSquarePercentage: 8  // Marqueurs plus grands et plus visibles
});
```

Des valeurs plus petites créent des marqueurs moins intrusifs qui conviennent mieux aux images détaillées. Des valeurs plus grandes rendent les marqueurs plus faciles à voir et à cliquer sur des images chargées d'éléments ou pour les utilisateurs sur appareils mobiles.

#### hasDarkBackground

Active le style en mode sombre lorsque votre page a un fond sombre.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    hasDarkBackground: true
});
```

#### commentCountUpdated

Une fonction de rappel qui se déclenche chaque fois que le nombre de commentaires change. Ceci est utile pour mettre à jour des éléments d'interface utilisateur comme des badges ou les titres de page.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
```

### Inherited Configuration Options

Puisque Image Chat étend le widget de commentaires standard, vous pouvez utiliser n'importe quelle option de configuration du widget FastComments de base. Voici quelques options couramment utilisées :

#### locale

Définissez la langue de l'interface du widget. FastComments prend en charge des dizaines de langues.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    locale: 'es'  // Espagnol
});
```

#### readonly

Rendez toutes les conversations en lecture seule. Les utilisateurs peuvent voir les marqueurs et discussions existants mais ne peuvent pas en créer de nouveaux ni répondre.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    readonly: true
});
```

#### sso and simpleSSO

Intégrez votre système d'authentification en utilisant Single Sign-On.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    sso: {
        // Configuration SSO
    }
});
```

Consultez la documentation SSO pour les détails complets sur les options d'authentification.

#### maxReplyDepth

Contrôlez combien de niveaux de réponses peuvent être imbriqués. Par défaut, Image Chat définit ceci à 0, ce qui signifie que tous les commentaires sont plats (pas de réponses imbriquées). Vous pouvez changer ceci si vous souhaitez des conversations en fils.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Autoriser 3 niveaux d'imbrication
});
```

### Internal Configuration

Ces options sont définies automatiquement par Image Chat et ne doivent pas être remplacées :

The `productId` is automatically set to `2` for Image Chat. The `floating-chat` extension is automatically loaded to provide the chat window functionality. The widget automatically detects mobile devices (screens under 768px wide) and adjusts the UI accordingly with fullscreen chat windows.

### Target Element Flexibility

Le premier paramètre de `FastCommentsImageChat` peut être soit un élément `<img>` directement, soit un élément conteneur contenant une image :

```javascript
// Élément image direct
FastCommentsImageChat(document.getElementById('my-image'), config);

// Conteneur contenant l'image
FastCommentsImageChat(document.querySelector('.image-wrapper'), config);
```

Le widget trouvera automatiquement l'image si vous passez un élément conteneur.

### Complete Example

Voici un exemple montrant plusieurs options de configuration ensemble :

```javascript
FastCommentsImageChat(document.getElementById('product-image'), {
    tenantId: 'demo',
    urlId: 'product-v2-main',
    chatSquarePercentage: 6,
    hasDarkBackground: false,
    locale: 'en',
    commentCountUpdated: function(count) {
        document.title = count > 0 ? `(${count}) Product Photo` : 'Product Photo';
    },
    sso: {
        // Votre configuration SSO
    },
    maxReplyDepth: 1
});
```

Pour une liste complète de toutes les options de configuration disponibles héritées du widget de base, consultez la documentation principale de configuration FastComments.
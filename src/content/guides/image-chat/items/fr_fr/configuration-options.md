### Aperçu

FastComments Image Chat étend le widget de commentaires standard FastComments, donc il hérite de toutes les options de configuration du widget de base tout en ajoutant quelques-unes spécifiques aux annotations d'image.

### Configuration requise

#### tenantId

Votre Tenant ID FastComments est requis. Vous pouvez le trouver dans votre [FastComments dashboard](https://fastcomments.com/auth/my-account/api-secret).

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo'
});
```

### Options spécifiques à Image Chat

#### urlId

Par défaut, Image Chat génère un identifiant unique pour chaque conversation basé sur l'URL de la page, la source de l'image et les coordonnées X/Y. Vous pouvez remplacer cela avec un `urlId` personnalisé.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    urlId: 'my-custom-image-id'
});
```

Ceci est utile lorsque la structure de vos URL peut changer mais que vous souhaitez conserver les mêmes conversations, ou lorsque vous voulez partager des annotations entre plusieurs pages.

#### chatSquarePercentage

Contrôle la taille des marqueurs cliquables en pourcentage de la largeur de l'image. La valeur par défaut est 5 %, ce qui signifie que chaque marqueur occupe 5 % de la largeur de l'image.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    chatSquarePercentage: 8  // Marqueurs plus grands et plus visibles
});
```

Des valeurs plus petites créent des marqueurs moins intrusifs, mieux adaptés aux images détaillées. Des valeurs plus grandes rendent les marqueurs plus faciles à voir et à cliquer sur des images chargées ou pour les utilisateurs sur appareils mobiles.

#### hasDarkBackground

Active le style en mode sombre lorsque votre page a un arrière-plan foncé.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    hasDarkBackground: true
});
```

#### commentCountUpdated

Une fonction de rappel qui est déclenchée chaque fois que le nombre de commentaires change. Cela est utile pour mettre à jour des éléments d'interface tels que des badges ou le titre de la page.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
```

### Options de configuration héritées

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

Rendre toutes les conversations en lecture seule. Les utilisateurs peuvent voir les marqueurs et discussions existants mais ne peuvent pas en créer de nouveaux ni répondre.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    readonly: true
});
```

#### sso and simpleSSO

Intégrez-vous à votre système d'authentification en utilisant l'authentification unique (SSO).

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    sso: {
        // Configuration SSO
    }
});
```

Consultez la documentation SSO pour tous les détails sur les options d'authentification.

#### maxReplyDepth

Contrôlez combien de niveaux de profondeur les réponses peuvent atteindre. Par défaut, Image Chat définit ceci à 0, ce qui signifie que tous les commentaires sont plats (pas de réponses en fil). Vous pouvez changer cela si vous souhaitez des conversations en thread.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Autoriser 3 niveaux d'imbrication
});
```

### Configuration interne

Ces options sont automatiquement définies par Image Chat et ne doivent pas être remplacées :

Le `productId` est automatiquement défini sur `2` pour Image Chat. L'extension `floating-chat` est automatiquement chargée pour fournir la fonctionnalité de fenêtre de discussion. Le widget détecte automatiquement les appareils mobiles (écrans de moins de 768px de large) et ajuste l'interface en conséquence avec des fenêtres de discussion en plein écran.

### Flexibilité de l'élément cible

Le premier paramètre de `FastCommentsImageChat` peut être soit un élément `<img>` directement, soit un élément conteneur contenant une image :

```javascript
// Élément <img> direct
FastCommentsImageChat(document.getElementById('my-image'), config);

// Conteneur contenant une image
FastCommentsImageChat(document.querySelector('.image-wrapper'), config);
```

Le widget trouvera automatiquement l'image si vous passez un élément conteneur.

### Exemple complet

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
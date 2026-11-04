### Vue d'ensemble

FastComments Collab Chat étend le widget de commentaires standard FastComments, il hérite donc de toutes les options de configuration du widget de base tout en ajoutant quelques options spécifiques aux annotations de texte.

### Configuration requise

#### tenantId

Votre ID de locataire FastComments est requis. Vous pouvez le trouver dans votre [tableau de bord FastComments](https://fastcomments.com/auth/my-account/api-secret).

[inline-code-attrs-start title = "Exemple de configuration"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo'
});
[inline-code-end]

### Options spécifiques à Collab Chat

#### urlId

Par défaut, Collab Chat génère un identifiant unique pour chaque conversation basé sur l'URL de la page, le chemin DOM de l'élément et la plage de texte sélectionnée. Vous pouvez le remplacer par un `urlId` personnalisé.

[inline-code-attrs-start title = "Exemple de configuration"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    urlId: 'my-custom-page-id'
});
[inline-code-end]

C'est utile lorsque la structure de votre URL peut changer mais que vous souhaitez conserver les mêmes conversations, ou lorsque vous voulez partager des annotations sur plusieurs pages.

#### topBarTarget

Contrôle l'affichage de la barre supérieure qui affiche le nombre d'utilisateurs et le nombre de discussions. Définissez-le sur `null` pour désactiver complètement la barre supérieure, ou fournissez un élément DOM pour l'afficher à un emplacement spécifique.

[inline-code-attrs-start title = "Exemple de configuration"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Disable top bar
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: null
});

// Render top bar in custom location
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: document.getElementById('custom-header')
});
[inline-code-end]

#### hasDarkBackground

Active le style en mode sombre lorsque votre page a un arrière-plan sombre. Cette détection est automatique, mais il peut être souhaitable de la remplacer.

[inline-code-attrs-start title = "Exemple de configuration"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    hasDarkBackground: true
});
[inline-code-end]

#### commentCountUpdated

Une fonction de rappel qui se déclenche chaque fois que le nombre de commentaires change. C'est utile pour mettre à jour des éléments d'interface comme des badges ou des titres de page.

[inline-code-attrs-start title = "Exemple de configuration"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
[inline-code-end]

### Options de configuration héritées

Comme Collab Chat étend le widget de commentaires standard, vous pouvez utiliser n'importe quelle option de configuration du widget FastComments de base. Voici quelques options couramment utilisées :

#### locale

Définit la langue de l'interface du widget. FastComments prend en charge des dizaines de langues.

[inline-code-attrs-start title = "Exemple de configuration"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    locale: 'es'  // Spanish
});
[inline-code-end]

#### readonly

Rend toutes les conversations en lecture seule. Les utilisateurs peuvent consulter les annotations existantes mais ne peuvent pas en créer de nouvelles ni y répondre.

[inline-code-attrs-start title = "Exemple de configuration"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    readonly: true
});
[inline-code-end]

#### sso et simpleSSO

Intégrez votre système d'authentification à l'aide de l'authentification unique (Single Sign-On).

[inline-code-attrs-start title = "Exemple de configuration"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    sso: {
        // SSO configuration
    }
});
[inline-code-end]

Consultez la documentation SSO pour plus de détails sur les options d'authentification.

#### maxReplyDepth

Contrôle la profondeur des réponses imbriquées. Par défaut, Collab Chat définit cette valeur sur 0, ce qui signifie que tous les commentaires sont à plat (pas de réponses imbriquées). Vous pouvez modifier cette valeur si vous souhaitez des conversations en fil de discussion.

[inline-code-attrs-start title = "Exemple de configuration"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Allow 3 levels of nesting
});
[inline-code-end]

### Configuration interne

Ces options sont définies automatiquement par Collab Chat et ne doivent pas être remplacées :

Le `productId` est automatiquement défini sur `3` pour Collab Chat. L'extension `floating-chat` est automatiquement chargée pour fournir la fonctionnalité de fenêtre de discussion. Le widget détecte automatiquement les appareils mobiles (écrans de moins de 768px de large) et adapte l'interface en conséquence.

### Exemple complet

Voici un exemple montrant plusieurs options de configuration ensemble :

[inline-code-attrs-start title = "Exemple de configuration"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(document.getElementById('article'), {
    tenantId: 'demo',
    urlId: 'my-article-v2',
    hasDarkBackground: false,
    locale: 'en',
    topBarTarget: document.getElementById('custom-header'),
    commentCountUpdated: function(count) {
        document.title = count > 0 ? `(${count}) My Article` : 'My Article';
    },
    sso: {
        // Your SSO config
    },
    maxReplyDepth: 1
});
[inline-code-end]

Pour une liste complète de toutes les options de configuration disponibles héritées du widget de base, consultez la documentation principale de configuration de FastComments.
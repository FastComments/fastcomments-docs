### Vue d'ensemble

FastComments Collab Chat étend le widget de commentaires standard FastComments, il hérite donc de toutes les options de configuration du widget de base tout en ajoutant quelques options spécifiques aux annotations de texte.

### Configuration requise

#### tenantId

Votre ID de locataire FastComments est requis. Vous pouvez le trouver dans votre [FastComments dashboard](https://fastcomments.com/auth/my-account/api-secret).

[inline-code-attrs-start title = "Exemple de configuration"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo'
});
[inline-code-end]

### Options spécifiques à Collab Chat

#### urlId

Par défaut, Collab Chat génère un identifiant unique pour chaque conversation basé sur l'URL de la page, le chemin DOM vers l'élément et la plage de texte sélectionnée. Vous pouvez remplacer ceci par un `urlId` personnalisé.

[inline-code-attrs-start title = "Exemple de configuration"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    urlId: 'my-custom-page-id'
});
[inline-code-end]

Ceci est utile lorsque la structure de vos URL peut changer mais que vous souhaitez conserver les mêmes conversations, ou lorsque vous voulez partager des annotations entre plusieurs pages.

#### topBarTarget

Contrôle l'affichage de la barre supérieure qui affiche le nombre d'utilisateurs et le nombre de discussions. Définissez sur `null` pour désactiver complètement la barre supérieure, ou fournissez un élément DOM pour l'afficher à un emplacement spécifique.

[inline-code-attrs-start title = "Exemple de configuration"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Désactiver la barre supérieure
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: null
});

// Afficher la barre supérieure dans un emplacement personnalisé
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: document.getElementById('custom-header')
});
[inline-code-end]

#### hasDarkBackground

Active le style en mode sombre lorsque votre page a un fond sombre. Cette détection est automatique, mais il peut être souhaitable de la remplacer.

[inline-code-attrs-start title = "Exemple de configuration"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    hasDarkBackground: true
});
[inline-code-end]

#### commentCountUpdated

Une fonction de rappel qui est déclenchée chaque fois que le nombre de commentaires change. Utile pour mettre à jour des éléments d'interface tels que des badges ou le titre de la page.

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
    locale: 'es'  // Espagnol
});
[inline-code-end]

#### readonly

Rendre toutes les conversations en lecture seule. Les utilisateurs peuvent voir les annotations existantes mais ne peuvent pas en créer de nouvelles ou répondre.

[inline-code-attrs-start title = "Exemple de configuration"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    readonly: true
});
[inline-code-end]

#### sso and simpleSSO

Intégrez votre système d'authentification via Single Sign-On.

[inline-code-attrs-start title = "Exemple de configuration"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    sso: {
        // Configuration SSO
    }
});
[inline-code-end]

Consultez la documentation SSO pour les détails complets sur les options d'authentification.

#### maxReplyDepth

Contrôlez la profondeur des niveaux de réponses. Par défaut, Collab Chat définit cela à 0, ce qui signifie que tous les commentaires sont plats (pas de réponses imbriquées). Vous pouvez modifier cela si vous souhaitez des conversations avec fils de discussion.

[inline-code-attrs-start title = "Exemple de configuration"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Autoriser 3 niveaux d'imbrication
});
[inline-code-end]

### Configuration interne

Ces options sont automatiquement définies par Collab Chat et ne doivent pas être remplacées :

Le `productId` est automatiquement défini sur `3` pour Collab Chat. L'extension `floating-chat` est automatiquement chargée pour fournir la fonctionnalité de fenêtre de chat. Le widget détecte automatiquement les appareils mobiles (écrans de moins de 768px de large) et ajuste l'interface en conséquence.

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
        // Votre configuration SSO
    },
    maxReplyDepth: 1
});
[inline-code-end]

Pour une liste complète de toutes les options de configuration disponibles héritées du widget de base, consultez la documentation de configuration principale de FastComments.
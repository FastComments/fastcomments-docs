### Aperçu de l'API

Collab Chat fournit trois points de terminaison REST API pour gérer les conversations textuelles de manière programmatique. Ces points de terminaison vous permettent de récupérer, créer et supprimer des annotations sans utiliser le widget du navigateur.

Ce sont des points de terminaison publics qui authentifient les utilisateurs via les cookies du navigateur. Ils n'utilisent pas de clés API. Les utilisateurs doivent être connectés à FastComments dans leur navigateur pour accéder à ces points de terminaison.

### URL de base

Tous les points de terminaison de l'API Collab Chat sont sous :

[inline-code-attrs-start title = 'URL de base'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/comment-collab-chats
[inline-code-end]

### Authentification

Ces points de terminaison authentifient les utilisateurs via les cookies du navigateur. Ils n'utilisent pas de clés API. Les utilisateurs doivent être connectés à FastComments dans leur navigateur pour accéder à ces points de terminaison.

### Récupérer toutes les conversations

Récupère toutes les conversations textuelles pour une page spécifique.

#### Point de terminaison

[inline-code-attrs-start title = 'Point de terminaison GET'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET /comment-collab-chats/:tenantId?urlId=<urlId>
[inline-code-end]

#### Paramètres

`tenantId` (paramètre de chemin, requis) est votre FastComments Tenant ID.

`urlId` (paramètre de requête, requis) est l'identifiant de la page pour laquelle vous voulez récupérer les conversations.

#### Réponse

La réponse inclut le statut de l'API, les informations de session de l'utilisateur actuel si authentifié, un tableau de conversations avec leurs IDs, URLs et plages de texte, un identifiant d'URL nettoyé, un indicateur indiquant si l'utilisateur actuel est administrateur du site ou modérateur, et les détails de connexion WebSocket pour la synchronisation en direct incluant `tenantIdWS`, `urlIdWS`, et `userIdWS`.

#### Exemple de requête

[inline-code-attrs-start title = 'Exemple de requête GET'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl "https://fastcomments.com/comment-collab-chats/demo?urlId=my-article-page" \
  --cookie "your-session-cookie"
[inline-code-end]

#### Exemple de réponse

[inline-code-attrs-start title = 'Exemple de réponse GET'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "user": {
    "id": "user123",
    "username": "john_doe"
  },
  "conversations": [
    {
      "_id": "conv123",
      "urlId": "my-article-page:p:0:15,0:45{abc123}",
      "range": "0:15,0:45{abc123}"
    },
    {
      "_id": "conv456",
      "urlId": "my-article-page:h1:5:20,5:35{def456}",
      "range": "5:20,5:35{def456}"
    }
  ],
  "urlIdClean": "my-article-page",
  "isSiteAdmin": false,
  "tenantIdWS": "demo",
  "urlIdWS": "my-article-page",
  "userIdWS": "user123"
}
[inline-code-end]

### Créer une conversation

Créer une nouvelle conversation textuelle pour une sélection de texte spécifique.

#### Point de terminaison

[inline-code-attrs-start title = 'Point de terminaison POST'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
POST /comment-collab-chats/:tenantId
[inline-code-end]

#### Paramètres

`tenantId` (paramètre de chemin, requis) est votre FastComments Tenant ID.

Le corps de la requête doit être au format JSON et inclure les champs requis suivants.

`urlId` (string, requis) est l'identifiant de la page de base.

`urlIdWithRange` (string, requis) correspond à l'URL combinée avec la plage de texte, par exemple `my-page:p:0:15,0:45{abc123}`.

`pageTitle` (string, requis) est le titre de la page.

`selector` (string, requis) est le chemin DOM vers l'élément contenant le texte sélectionné.

`range` (string, requis) est la plage de texte sérialisée au format `startOffset:endOffset,startOffset:endOffset{checksum}`.

`createdFromCommentId` (string, requis) est l'ID du commentaire qui a initié cette discussion.

`broadcastId` (string, requis) est un UUID pour la synchronisation en direct afin d'empêcher les effets d'écho.

#### Réponse

La réponse inclut le statut de l'API et l'ID de la conversation nouvellement créée.

#### Exemple de requête

[inline-code-attrs-start title = 'Exemple de requête POST'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -X POST "https://fastcomments.com/comment-collab-chats/demo" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "urlId": "my-article-page",
    "urlIdWithRange": "my-article-page:p:0:15,0:45{abc123}",
    "pageTitle": "My Article Title",
    "selector": "div#article > p:nth-child(2)",
    "range": "0:15,0:45{abc123}",
    "createdFromCommentId": "comment789",
    "broadcastId": "550e8400-e29b-41d4-a716-446655440000"
  }'
[inline-code-end]

#### Exemple de réponse

[inline-code-attrs-start title = 'Exemple de réponse POST'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "createdChatId": "conv789"
}
[inline-code-end]

### Supprimer une conversation

Supprime une conversation textuelle existante. Ce point de terminaison nécessite les permissions d'administrateur ou de modérateur, ou la conversation doit avoir été créée par l'utilisateur authentifié.

#### Point de terminaison

[inline-code-attrs-start title = 'Point de terminaison DELETE'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
DELETE /comment-collab-chats/:tenantId/:chatId
[inline-code-end]

#### Paramètres

`tenantId` (paramètre de chemin, requis) est votre FastComments Tenant ID.

`chatId` (paramètre de chemin, requis) est l'ID de la conversation à supprimer.

`broadcastId` (corps de la requête, requis) est un UUID pour la synchronisation en direct.

#### Exemple de requête

[inline-code-attrs-start title = 'Exemple de requête DELETE'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -X DELETE "https://fastcomments.com/comment-collab-chats/demo/conv789" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "broadcastId": "550e8400-e29b-41d4-a716-446655440001"
  }'
[inline-code-end]

#### Exemple de réponse

[inline-code-attrs-start title = 'Exemple de réponse DELETE'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success"
}
[inline-code-end]

### Limitation de débit

Ces points de terminaison sont soumis à la limitation de débit standard de l'API FastComments. Le middleware de limitation s'applique par locataire afin d'empêcher les abus tout en permettant des modèles d'utilisation normaux.

### Réponses d'erreur

Tous les points de terminaison renvoient des codes de statut HTTP standard. Une réponse 400 indique des paramètres de requête invalides. Une réponse 401 signifie que l'authentification a échoué. Une réponse 403 indique des permissions insuffisantes. Une réponse 404 signifie que la conversation est introuvable. Une réponse 429 indique que la limite de taux a été dépassée.

Les réponses d'erreur incluent un corps JSON avec des détails :

[inline-code-attrs-start title = 'Exemple de réponse d\'erreur'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "error",
  "message": "Conversation not found"
}
[inline-code-end]

### Suivi de l'utilisation

La création de conversations incrémente votre métrique d'utilisation `conversationCreateCount`. Toute activité de synchronisation WebSocket incrémente `pubSubMessageCount` et `pubSubBandwidth`. Vous pouvez surveiller ces métriques dans votre tableau de bord FastComments sous les analyses d'utilisation.
### Aperçu de l'API

Collab Chat fournit trois points d'API REST pour gérer des conversations textuelles de manière programmatique. Ces points d'accès vous permettent de récupérer, créer et supprimer des annotations sans utiliser le widget du navigateur.

Ce sont des points d'accès publics qui authentifient les utilisateurs via les cookies du navigateur. Ils n'utilisent pas de clés API. Les utilisateurs doivent être connectés à FastComments dans leur navigateur pour accéder à ces points d'accès.

### URL de base

Tous les points d'API de Collab Chat se trouvent sous :

[inline-code-attrs-start title = 'URL de base'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/comment-collab-chats
[inline-code-end]

### Authentification

Ces points d'accès authentifient les utilisateurs via les cookies du navigateur. Ils n'utilisent pas de clés API. Les utilisateurs doivent être connectés à FastComments dans leur navigateur pour accéder à ces points d'accès.

### Récupérer toutes les conversations

Récupère toutes les conversations textuelles pour une page spécifique.

#### Point d'accès

[inline-code-attrs-start title = 'Point d\'accès GET'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET /comment-collab-chats/:tenantId?urlId=<urlId>
[inline-code-end]

#### Paramètres

`tenantId` (paramètre de chemin, requis) est votre ID de tenant FastComments.

`urlId` (paramètre de requête, requis) est l'identifiant de la page pour laquelle vous souhaitez récupérer les conversations.

#### Réponse

La réponse inclut le statut de l'API, les informations de session de l'utilisateur actuel si authentifié, un tableau de conversations avec leurs ID, URL et plages de texte, un identifiant d'URL nettoyé, un indicateur indiquant si l'utilisateur actuel est un administrateur ou un modérateur du site, et les détails de connexion WebSocket pour la synchronisation en direct incluant `tenantIdWS`, `urlIdWS` et `userIdWS`.

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

Crée une nouvelle conversation textuelle pour une sélection de texte spécifique.

#### Point d'accès

[inline-code-attrs-start title = 'Point d\'accès POST'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
POST /comment-collab-chats/:tenantId
[inline-code-end]

#### Paramètres

`tenantId` (paramètre de chemin, requis) est votre ID de tenant FastComments.

Le corps de la requête doit être en JSON et inclure ces champs requis.

`urlId` (chaîne, requis) est l'identifiant de base de la page.

`urlIdWithRange` (chaîne, requis) est l'URL combinée avec la plage de texte, par exemple `my-page:p:0:15,0:45{abc123}`.

`pageTitle` (chaîne, requis) est le titre de la page.

`selector` (chaîne, requis) est le chemin DOM vers l'élément contenant le texte sélectionné.

`range` (chaîne, requis) est la plage de texte sérialisée au format `startOffset:endOffset,startOffset:endOffset{checksum}`.

`createdFromCommentId` (chaîne, requis) est l'ID du commentaire qui a initié ce chat.

`broadcastId` (chaîne, requis) est un UUID pour la synchronisation en direct afin de prévenir les effets d'écho.

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

Supprime une conversation textuelle existante. Ce point d'accès nécessite des permissions d'administrateur ou de modérateur, ou la conversation doit avoir été créée par l'utilisateur authentifié.

#### Point d'accès

[inline-code-attrs-start title = 'Point d\'accès DELETE'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
DELETE /comment-collab-chats/:tenantId/:chatId
[inline-code-end]

#### Paramètres

`tenantId` (paramètre de chemin, requis) est votre ID de tenant FastComments.

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

Ces points d'accès sont soumis à la limitation de débit standard de l'API FastComments. Le middleware de limitation de débit s'applique par tenant pour prévenir les abus tout en permettant des modèles d'utilisation normaux.

### Réponses d'erreur

Tous les points d'accès renvoient des codes de statut HTTP standards. Une réponse 400 indique des paramètres de requête invalides. Une réponse 401 signifie que l'authentification a échoué. Une réponse 403 indique des permissions insuffisantes. Une réponse 404 signifie que la conversation n'a pas été trouvée. Une réponse 429 indique que la limite de débit a été dépassée.

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
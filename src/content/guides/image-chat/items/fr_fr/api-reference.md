### API Overview

Image Chat fournit trois points de terminaison REST API pour gérer les conversations d'image de manière programmatique. Ces points de terminaison vous permettent de récupérer, créer et supprimer des marqueurs sans utiliser le widget du navigateur.

Tous les points de terminaison API requièrent une authentification et suivent les modèles standard de l'API FastComments. Ce sont des points de terminaison publics qui s'authentifient via des cookies de navigateur, et non via des clés API.

### Base URL

Tous les points de terminaison de l'API Image Chat se trouvent sous :

```
https://fastcomments.com/comment-image-chats
```

### Authentication

Ces points de terminaison authentifient les utilisateurs via des cookies de navigateur. Ils n'utilisent pas de clés API. Les utilisateurs doivent être connectés à FastComments dans leur navigateur pour accéder à ces points de terminaison.

### Get All Conversations

Récupère toutes les conversations d'image pour une image donnée.

#### Endpoint

```
GET /comment-image-chats/:tenantId?urlId=<urlId>
```

#### Parameters

`tenantId` (path parameter, required) est votre FastComments Tenant ID.

`urlId` (query parameter, required) est l'identifiant de l'image pour lequel vous souhaitez récupérer les conversations.

#### Response

La réponse inclut le statut de l'API, les informations de session de l'utilisateur courant si authentifié, un tableau de conversations avec leurs IDs, URLs et coordonnées X/Y, un identifiant d'URL nettoyé, un indicateur précisant si l'utilisateur courant est administrateur ou modérateur du site, et les détails de connexion WebSocket pour la synchronisation en direct incluant `tenantIdWS`, `urlIdWS` et `userIdWS`.

#### Example Request

```bash
curl "https://fastcomments.com/comment-image-chats/demo?urlId=my-product-image" \
  --cookie "your-session-cookie"
```

#### Example Response

```json
{
  "status": "success",
  "user": {
    "id": "user123",
    "username": "john_doe"
  },
  "conversations": [
    {
      "_id": "conv123",
      "urlId": "my-product-image:/images/product.jpg:25:30",
      "x": 25.5,
      "y": 30.2
    },
    {
      "_id": "conv456",
      "urlId": "my-product-image:/images/product.jpg:60:45",
      "x": 60.8,
      "y": 45.1
    }
  ],
  "urlIdClean": "my-product-image",
  "isSiteAdmin": false,
  "tenantIdWS": "demo",
  "urlIdWS": "my-product-image",
  "userIdWS": "user123"
}
```

### Create Conversation

Créer une nouvelle conversation d'image pour un emplacement spécifique sur une image.

#### Endpoint

```
POST /comment-image-chats/:tenantId
```

#### Parameters

`tenantId` (path parameter, required) est votre FastComments Tenant ID.

Le corps de la requête doit être en JSON et inclure les champs requis suivants.

`urlId` (string, required) est l'identifiant de la page de base.

`windowUrlId` (string, required) est l'URL combinée avec la source de l'image et les coordonnées, par exemple `my-page:/images/photo.jpg:25.5:30.2`.

`pageTitle` (string, required) est le titre de la page.

`src` (string, required) est l'URL source de l'image.

`x` (number, required) est la coordonnée X en pourcentage (0-100).

`y` (number, required) est la coordonnée Y en pourcentage (0-100).

`createdFromCommentId` (string, required) est l'ID du commentaire qui a initié ce chat.

`broadcastId` (string, required) est un UUID pour la synchronisation en direct afin d'éviter les effets d'écho.

#### Response

La réponse inclut le statut de l'API et l'ID de la conversation nouvellement créée.

#### Example Request

```bash
curl -X POST "https://fastcomments.com/comment-image-chats/demo" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "urlId": "my-product-image",
    "windowUrlId": "my-product-image:/images/product.jpg:25.5:30.2",
    "pageTitle": "Product Gallery",
    "src": "/images/product.jpg",
    "x": 25.5,
    "y": 30.2,
    "createdFromCommentId": "comment789",
    "broadcastId": "550e8400-e29b-41d4-a716-446655440000"
  }'
```

#### Example Response

```json
{
  "status": "success",
  "createdChatId": "conv789"
}
```

### Delete Conversation

Supprime une conversation d'image existante. Ce point de terminaison nécessite des permissions d'administrateur ou de modérateur, ou la conversation doit avoir été créée par l'utilisateur authentifié.

#### Endpoint

```
DELETE /comment-image-chats/:tenantId/:chatId
```

#### Parameters

`tenantId` (path parameter, required) est votre FastComments Tenant ID.

`chatId` (path parameter, required) est l'ID de la conversation à supprimer.

`broadcastId` (request body, required) est un UUID pour la synchronisation en direct.

#### Example Request

```bash
curl -X DELETE "https://fastcomments.com/comment-image-chats/demo/conv789" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "broadcastId": "550e8400-e29b-41d4-a716-446655440001"
  }'
```

#### Example Response

```json
{
  "status": "success"
}
```

### Coordinate System

Les coordonnées X et Y sont stockées en tant que pourcentages des dimensions de l'image. X représente la position horizontale depuis le bord gauche (0 = bord gauche, 100 = bord droit). Y représente la position verticale depuis le bord supérieur (0 = bord supérieur, 100 = bord inférieur).

Ces valeurs en pourcentage peuvent inclure des décimales pour plus de précision. Par exemple, x: 25.5 signifie 25.5% depuis le bord gauche de l'image.

### Rate Limiting

Ces points de terminaison sont soumis à la limitation de débit standard de l'API FastComments. Le middleware de limitation de débit s'applique par tenant pour prévenir les abus tout en permettant des schémas d'utilisation normaux.

### Error Responses

Tous les points de terminaison renvoient des codes de statut HTTP standards. Une réponse 400 indique des paramètres de requête invalides. Une réponse 401 signifie que l'authentification a échoué. Une réponse 403 indique des permissions insuffisantes. Une réponse 404 signifie que la conversation n'a pas été trouvée. Une réponse 429 indique que la limite de débit a été dépassée.

Les réponses d'erreur incluent un corps JSON avec des détails :

```json
{
  "status": "error",
  "message": "Conversation not found"
}
```

### Usage Tracking

La création de conversations incrémente votre métrique d'utilisation `conversationCreateCount`. Toute activité de synchronisation WebSocket incrémente `pubSubMessageCount` et `pubSubBandwidth`. Vous pouvez surveiller ces métriques dans votre tableau de bord FastComments sous usage analytics.
### Vue d'ensemble de l'API

Image Chat fournit trois points d'API REST pour gérer les conversations d'images de manière programmatique. Ces points d'accès vous permettent de récupérer, créer et supprimer des marqueurs sans utiliser le widget du navigateur.

Tous les points d'API nécessitent une authentification et suivent les modèles d'API FastComments standard. Ce sont des points d'accès publics qui s'authentifient via les cookies du navigateur, et non via des clés d'API.

### URL de base

Tous les points d'API Image Chat se trouvent sous :

```
https://fastcomments.com/comment-image-chats
```

### Authentification

Ces points d'accès authentifient les utilisateurs via les cookies du navigateur. Ils n'utilisent pas de clés d'API. Les utilisateurs doivent être connectés à FastComments dans leur navigateur pour accéder à ces points d'accès.

### Récupérer toutes les conversations

Récupère toutes les conversations d'images pour une image spécifique.

#### Point d'accès

```
GET /comment-image-chats/:tenantId?urlId=<urlId>
```

#### Paramètres

`tenantId` (paramètre de chemin, requis) est votre ID de tenant FastComments.

`urlId` (paramètre de requête, requis) est l'identifiant de l'image pour laquelle vous souhaitez récupérer les conversations.

#### Réponse

La réponse inclut le statut de l'API, les informations de session de l'utilisateur actuel si authentifié, un tableau de conversations avec leurs IDs, URLs et coordonnées X/Y, un identifiant d'URL nettoyé, un indicateur indiquant si l'utilisateur actuel est un administrateur ou un modérateur du site, et les détails de connexion WebSocket pour la synchronisation en direct, y compris `tenantIdWS`, `urlIdWS` et `userIdWS`.

#### Exemple de requête

```bash
curl "https://fastcomments.com/comment-image-chats/demo?urlId=my-product-image" \
  --cookie "your-session-cookie"
```

#### Exemple de réponse

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

### Créer une conversation

Crée une nouvelle conversation d'image pour un emplacement spécifique sur une image.

#### Point d'accès

```
POST /comment-image-chats/:tenantId
```

#### Paramètres

`tenantId` (paramètre de chemin, requis) est votre ID de tenant FastComments.

Le corps de la requête doit être en JSON et inclure ces champs requis.

`urlId` (chaîne, requis) est l'identifiant de la page de base.

`windowUrlId` (chaîne, requis) est l'URL combinée avec la source de l'image et les coordonnées, par exemple `my-page:/images/photo.jpg:25.5:30.2`.

`pageTitle` (chaîne, requis) est le titre de la page.

`src` (chaîne, requis) est l'URL source de l'image.

`x` (nombre, requis) est la coordonnée X en pourcentage (0-100).

`y` (nombre, requis) est la coordonnée Y en pourcentage (0-100).

`createdFromCommentId` (chaîne, requis) est l'ID du commentaire qui a initié cette conversation.

`broadcastId` (chaîne, requis) est un UUID pour la synchronisation en direct afin d'éviter les effets d'écho.

#### Réponse

La réponse inclut le statut de l'API et l'ID de la conversation nouvellement créée.

#### Exemple de requête

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

#### Exemple de réponse

```json
{
  "status": "success",
  "createdChatId": "conv789"
}
```

### Supprimer une conversation

Supprime une conversation d'image existante. Ce point d'accès nécessite des permissions d'administrateur ou de modérateur, ou la conversation doit avoir été créée par l'utilisateur authentifié.

#### Point d'accès

```
DELETE /comment-image-chats/:tenantId/:chatId
```

#### Paramètres

`tenantId` (paramètre de chemin, requis) est votre ID de tenant FastComments.

`chatId` (paramètre de chemin, requis) est l'ID de la conversation à supprimer.

`broadcastId` (corps de la requête, requis) est un UUID pour la synchronisation en direct.

#### Exemple de requête

```bash
curl -X DELETE "https://fastcomments.com/comment-image-chats/demo/conv789" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "broadcastId": "550e8400-e29b-41d4-a716-446655440001"
  }'
```

#### Exemple de réponse

```json
{
  "status": "success"
}
```

### Système de coordonnées

Les coordonnées X et Y sont stockées sous forme de pourcentages des dimensions de l'image. X représente la position horizontale à partir du bord gauche (0 = bord gauche, 100 = bord droit). Y représente la position verticale à partir du bord supérieur (0 = bord supérieur, 100 = bord inférieur).

Ces valeurs en pourcentage peuvent inclure des décimales pour plus de précision. Par exemple, x : 25.5 signifie 25,5 % à partir du bord gauche de l'image.

### Limitation de débit

Ces points d'accès sont soumis à la limitation de débit standard de l'API FastComments. Le middleware de limitation de débit s'applique par tenant pour prévenir les abus tout en permettant des modèles d'utilisation normaux.

### Réponses d'erreur

Tous les points d'accès renvoient des codes de statut HTTP standard. Une réponse 400 indique des paramètres de requête invalides. Une réponse 401 signifie que l'authentification a échoué. Une réponse 403 indique des permissions insuffisantes. Une réponse 404 signifie que la conversation n'a pas été trouvée. Une réponse 429 indique que la limite de débit a été dépassée.

Les réponses d'erreur incluent un corps JSON avec des détails :

```json
{
  "status": "error",
  "message": "Conversation not found"
}
```

### Suivi de l'utilisation

La création de conversations incrémente votre métrique d'utilisation `conversationCreateCount`. Toute activité de synchronisation WebSocket incrémente `pubSubMessageCount` et `pubSubBandwidth`. Vous pouvez surveiller ces métriques dans votre tableau de bord FastComments sous les analyses d'utilisation.
### Aperçu de l'API

Collab Chat fournit trois endpoints REST API pour gérer les conversations textuelles de manière programmatique. Ces endpoints vous permettent de récupérer, créer et supprimer des annotations sans utiliser le widget du navigateur.

Ce sont des points de terminaison publics qui authentifient les utilisateurs via les cookies du navigateur. Ils n'utilisent pas de clés API. Les utilisateurs doivent être connectés à FastComments dans leur navigateur pour accéder à ces endpoints.

### Base URL

All Collab Chat API endpoints are under:

[inline-code-attrs-start title = 'URL de base'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/comment-collab-chats
[inline-code-end]

### Authentication

These endpoints authenticate users via browser cookies. They do not use API keys. Users must be logged into FastComments in their browser to access these endpoints.

### Get All Conversations

Retrieve all text conversations for a specific page.

#### Endpoint

[inline-code-attrs-start title = 'Point de terminaison GET'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET /comment-collab-chats/:tenantId?urlId=<urlId>
[inline-code-end]

#### Parameters

`tenantId` (path parameter, required) is your FastComments Tenant ID.

`urlId` (query parameter, required) is the page identifier you want to retrieve conversations for.

#### Response

The response includes the API status, current user session information if authenticated, an array of conversations with their IDs, URLs, and text ranges, a cleaned URL identifier, a flag indicating if the current user is a site admin or moderator, and WebSocket connection details for live sync including `tenantIdWS`, `urlIdWS`, and `userIdWS`.

#### Example Request

[inline-code-attrs-start title = 'Exemple de requête GET'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl "https://fastcomments.com/comment-collab-chats/demo?urlId=my-article-page" \
  --cookie "your-session-cookie"
[inline-code-end]

#### Example Response

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

### Create Conversation

Create a new text conversation for a specific text selection.

#### Endpoint

[inline-code-attrs-start title = 'Point de terminaison POST'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
POST /comment-collab-chats/:tenantId
[inline-code-end]

#### Parameters

`tenantId` (path parameter, required) is your FastComments Tenant ID.

The request body must be JSON and include these required fields.

`urlId` (string, required) is the base page identifier.

`urlIdWithRange` (string, required) is the URL combined with the text range, for example `my-page:p:0:15,0:45{abc123}`.

`pageTitle` (string, required) is the title of the page.

`selector` (string, required) is the DOM path to the element containing the selected text.

`range` (string, required) is the serialized text range in the format `startOffset:endOffset,startOffset:endOffset{checksum}`.

`createdFromCommentId` (string, required) is the ID of the comment that initiated this chat.

`broadcastId` (string, required) is a UUID for live synchronization to prevent echo effects.

#### Response

The response includes the API status and the ID of the newly created conversation.

#### Example Request

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

#### Example Response

[inline-code-attrs-start title = 'Exemple de réponse POST'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "createdChatId": "conv789"
}
[inline-code-end]

### Delete Conversation

Delete an existing text conversation. This endpoint requires admin or moderator permissions, or the conversation must have been created by the authenticated user.

#### Endpoint

[inline-code-attrs-start title = 'Point de terminaison DELETE'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
DELETE /comment-collab-chats/:tenantId/:chatId
[inline-code-end]

#### Parameters

`tenantId` (path parameter, required) is your FastComments Tenant ID.

`chatId` (path parameter, required) is the ID of the conversation to delete.

`broadcastId` (request body, required) is a UUID for live synchronization.

#### Example Request

[inline-code-attrs-start title = 'Exemple de requête DELETE'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -X DELETE "https://fastcomments.com/comment-collab-chats/demo/conv789" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "broadcastId": "550e8400-e29b-41d4-a716-446655440001"
  }'
[inline-code-end]

#### Example Response

[inline-code-attrs-start title = 'Exemple de réponse DELETE'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success"
}
[inline-code-end]

### Rate Limiting

These endpoints are subject to standard FastComments API rate limiting. The rate limit middleware applies per-tenant to prevent abuse while allowing normal usage patterns.

### Error Responses

All endpoints return standard HTTP status codes. A 400 response indicates invalid request parameters. A 401 response means authentication failed. A 403 response indicates insufficient permissions. A 404 response means the conversation was not found. A 429 response indicates rate limit exceeded.

Error responses include a JSON body with details:

[inline-code-attrs-start title = 'Exemple de réponse d'erreur'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "error",
  "message": "Conversation not found"
}
[inline-code-end]

### Usage Tracking

Creating conversations increments your `conversationCreateCount` usage metric. All WebSocket sync activity increments `pubSubMessageCount` and `pubSubBandwidth`. You can monitor these metrics in your FastComments dashboard under usage analytics.

---
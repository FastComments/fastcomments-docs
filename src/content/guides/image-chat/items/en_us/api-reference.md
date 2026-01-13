### API Overview

Image Chat provides three REST API endpoints for managing image conversations programmatically. These endpoints allow you to retrieve, create, and delete markers without using the browser widget.

All API endpoints require authentication and follow the standard FastComments API patterns. These are public endpoints that authenticate via browser cookies, not API keys.

### Base URL

All Image Chat API endpoints are under:

```
https://fastcomments.com/comment-image-chats
```

### Authentication

These endpoints authenticate users via browser cookies. They do not use API keys. Users must be logged into FastComments in their browser to access these endpoints.

### Get All Conversations

Retrieve all image conversations for a specific image.

#### Endpoint

```
GET /comment-image-chats/:tenantId?urlId=<urlId>
```

#### Parameters

`tenantId` (path parameter, required) is your FastComments Tenant ID.

`urlId` (query parameter, required) is the image identifier you want to retrieve conversations for.

#### Response

The response includes the API status, current user session information if authenticated, an array of conversations with their IDs, URLs, and X/Y coordinates, a cleaned URL identifier, a flag indicating if the current user is a site admin or moderator, and WebSocket connection details for live sync including `tenantIdWS`, `urlIdWS`, and `userIdWS`.

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

Create a new image conversation for a specific location on an image.

#### Endpoint

```
POST /comment-image-chats/:tenantId
```

#### Parameters

`tenantId` (path parameter, required) is your FastComments Tenant ID.

The request body must be JSON and include these required fields.

`urlId` (string, required) is the base page identifier.

`windowUrlId` (string, required) is the URL combined with the image source and coordinates, for example `my-page:/images/photo.jpg:25.5:30.2`.

`pageTitle` (string, required) is the title of the page.

`src` (string, required) is the image source URL.

`x` (number, required) is the X coordinate as a percentage (0-100).

`y` (number, required) is the Y coordinate as a percentage (0-100).

`createdFromCommentId` (string, required) is the ID of the comment that initiated this chat.

`broadcastId` (string, required) is a UUID for live synchronization to prevent echo effects.

#### Response

The response includes the API status and the ID of the newly created conversation.

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

Delete an existing image conversation. This endpoint requires admin or moderator permissions, or the conversation must have been created by the authenticated user.

#### Endpoint

```
DELETE /comment-image-chats/:tenantId/:chatId
```

#### Parameters

`tenantId` (path parameter, required) is your FastComments Tenant ID.

`chatId` (path parameter, required) is the ID of the conversation to delete.

`broadcastId` (request body, required) is a UUID for live synchronization.

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

The X and Y coordinates are stored as percentages of the image dimensions. X represents the horizontal position from the left edge (0 = left edge, 100 = right edge). Y represents the vertical position from the top edge (0 = top edge, 100 = bottom edge).

These percentage values can include decimals for precision. For example, x: 25.5 means 25.5% from the left edge of the image.

### Rate Limiting

These endpoints are subject to standard FastComments API rate limiting. The rate limit middleware applies per-tenant to prevent abuse while allowing normal usage patterns.

### Error Responses

All endpoints return standard HTTP status codes. A 400 response indicates invalid request parameters. A 401 response means authentication failed. A 403 response indicates insufficient permissions. A 404 response means the conversation was not found. A 429 response indicates rate limit exceeded.

Error responses include a JSON body with details:

```json
{
  "status": "error",
  "message": "Conversation not found"
}
```

### Usage Tracking

Creating conversations increments your `conversationCreateCount` usage metric. All WebSocket sync activity increments `pubSubMessageCount` and `pubSubBandwidth`. You can monitor these metrics in your FastComments dashboard under usage analytics.

---
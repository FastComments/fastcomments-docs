### API 概述

Image Chat 提供三个用于以编程方式管理图片会话的 REST API 端点。 这些端点允许您在不使用浏览器小部件的情况下检索、创建和删除标记。

所有 API 端点都需要身份验证并遵循标准 FastComments API 模式。 这些是通过浏览器 cookie 进行身份验证的公共端点，而不是使用 API 密钥。

### 基础 URL

所有 Image Chat API 端点位于：

```
https://fastcomments.com/comment-image-chats
```

### 认证

这些端点通过浏览器 cookie 对用户进行认证。 它们不使用 API 密钥。 用户必须在其浏览器中登录 FastComments 才能访问这些端点。

### 获取所有会话

检索特定图片的所有图片会话。

#### 端点

```
GET /comment-image-chats/:tenantId?urlId=<urlId>
```

#### 参数

`tenantId`（路径参数，必需）是您的 FastComments 租户 ID。

`urlId`（查询参数，必需）是您要检索会话的图片标识符。

#### 响应

响应包含 API 状态、如果已认证则包含当前用户会话信息、带有其 ID、URL 和 X/Y 坐标的会话数组、清理后的 URL 标识符、指示当前用户是否为站点管理员或版主的标志，以及用于实时同步的 WebSocket 连接详细信息，包括 `tenantIdWS`、`urlIdWS` 和 `userIdWS`。

#### 示例请求

```bash
curl "https://fastcomments.com/comment-image-chats/demo?urlId=my-product-image" \
  --cookie "your-session-cookie"
```

#### 示例响应

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

### 创建会话

为图片上特定位置创建新的图片会话。

#### 端点

```
POST /comment-image-chats/:tenantId
```

#### 参数

`tenantId`（路径参数，必需）是您的 FastComments 租户 ID。

请求体必须为 JSON，并包含以下必需字段。

`urlId`（字符串，必需）是页面的基础标识符。

`windowUrlId`（字符串，必需）是与图片源和坐标组合的 URL，例如 `my-page:/images/photo.jpg:25.5:30.2`。

`pageTitle`（字符串，必需）是页面标题。

`src`（字符串，必需）是图片源 URL。

`x`（数字，必需）是以百分比表示的 X 坐标（0-100）。

`y`（数字，必需）是以百分比表示的 Y 坐标（0-100）。

`createdFromCommentId`（字符串，必需）是发起此聊天的评论 ID。

`broadcastId`（字符串，必需）是用于实时同步的 UUID，用于防止回声效应。

#### 响应

响应包括 API 状态和新创建会话的 ID。

#### 示例请求

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

#### 示例响应

```json
{
  "status": "success",
  "createdChatId": "conv789"
}
```

### 删除会话

删除现有的图片会话。 此端点需要管理员或版主权限，或者该会话必须由已认证的用户创建。

#### 端点

```
DELETE /comment-image-chats/:tenantId/:chatId
```

#### 参数

`tenantId`（路径参数，必需）是您的 FastComments 租户 ID。

`chatId`（路径参数，必需）是要删除的会话 ID。

`broadcastId`（请求体，必需）是用于实时同步的 UUID。

#### 示例请求

```bash
curl -X DELETE "https://fastcomments.com/comment-image-chats/demo/conv789" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "broadcastId": "550e8400-e29b-41d4-a716-446655440001"
  }'
```

#### 示例响应

```json
{
  "status": "success"
}
```

### 坐标系统

X 和 Y 坐标以图片尺寸的百分比存储。X 表示从左边缘起的水平位置（0 = 左边缘，100 = 右边缘）。Y 表示从上边缘起的垂直位置（0 = 上边缘，100 = 底部）。

这些百分比值可以包含小数以提高精度。例如，x: 25.5 表示从图片左边缘起 25.5%。

### 速率限制

这些端点受标准 FastComments API 的速率限制约束。速率限制中间件按租户应用，以防止滥用，同时允许正常使用模式。

### 错误响应

所有端点都会返回标准的 HTTP 状态码。400 响应表示请求参数无效。401 响应表示身份验证失败。403 响应表示权限不足。404 响应表示未找到会话。429 响应表示超出速率限制。

错误响应包含带有详细信息的 JSON 主体：

```json
{
  "status": "error",
  "message": "Conversation not found"
}
```

### 使用跟踪

创建会话会增加您的 `conversationCreateCount` 使用指标。所有 WebSocket 同步活动会增加 `pubSubMessageCount` 和 `pubSubBandwidth`。您可以在 FastComments 仪表板的使用分析中监控这些指标。

---
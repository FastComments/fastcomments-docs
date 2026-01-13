---
### API 概览

Collab Chat 提供三个用于以编程方式管理文本对话的 REST API 端点。 这些端点允许您在不使用浏览器小部件的情况下检索、创建和删除注释。

这些是通过浏览器 Cookie 对用户进行身份验证的公共端点。 它们不使用 API 密钥。用户必须在其浏览器中登录 FastComments 才能访问这些端点。

### 基本 URL

所有 Collab Chat API 端点都位于：

[inline-code-attrs-start title = '基本 URL'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/comment-collab-chats
[inline-code-end]

### 身份验证

这些端点通过浏览器 Cookie 对用户进行身份验证。 它们不使用 API 密钥。用户必须在其浏览器中登录 FastComments 才能访问这些端点。

### 获取所有会话

检索特定页面的所有文本会话。

#### 端点

[inline-code-attrs-start title = 'GET 端点'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET /comment-collab-chats/:tenantId?urlId=<urlId>
[inline-code-end]

#### 参数

`tenantId`（路径参数，必填）是您的 FastComments 租户 ID。

`urlId`（查询参数，必填）是您要检索会话的页面标识符。

#### 响应

响应包括 API 状态、如果已通过身份验证则包含的当前用户会话信息、带有其 ID、URL 和文本范围的会话数组、已清理的 URL 标识符、指示当前用户是否为站点管理员或版主的标志，以及用于实时同步的 WebSocket 连接详细信息，包括 `tenantIdWS`、`urlIdWS` 和 `userIdWS`。

#### 示例请求

[inline-code-attrs-start title = 'GET 请求示例'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl "https://fastcomments.com/comment-collab-chats/demo?urlId=my-article-page" \
  --cookie "your-session-cookie"
[inline-code-end]

#### 示例响应

[inline-code-attrs-start title = 'GET 响应示例'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### 创建会话

为特定文本选择创建新的文本会话。

#### 端点

[inline-code-attrs-start title = 'POST 端点'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
POST /comment-collab-chats/:tenantId
[inline-code-end]

#### 参数

`tenantId`（路径参数，必填）是您的 FastComments 租户 ID。

请求体必须为 JSON 并包含以下必填字段。

`urlId`（字符串，必填）是基础页面标识符。

`urlIdWithRange`（字符串，必填）是与文本范围组合的 URL，例如 `my-page:p:0:15,0:45{abc123}`。

`pageTitle`（字符串，必填）是页面标题。

`selector`（字符串，必填）是包含所选文本的元素的 DOM 路径。

`range`（字符串，必填）是序列化的文本范围，格式为 `startOffset:endOffset,startOffset:endOffset{checksum}`。

`createdFromCommentId`（字符串，必填）是发起此聊天的评论 ID。

`broadcastId`（字符串，必填）是用于实时同步以防止回声效应的 UUID。

#### 响应

响应包括 API 状态和新创建会话的 ID。

#### 示例请求

[inline-code-attrs-start title = 'POST 请求示例'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
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

#### 示例响应

[inline-code-attrs-start title = 'POST 响应示例'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "createdChatId": "conv789"
}
[inline-code-end]

### 删除会话

删除现有的文本会话。此端点需要管理员或版主权限，或该会话必须由经过身份验证的用户创建。

#### 端点

[inline-code-attrs-start title = 'DELETE 端点'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
DELETE /comment-collab-chats/:tenantId/:chatId
[inline-code-end]

#### 参数

`tenantId`（路径参数，必填）是您的 FastComments 租户 ID。

`chatId`（路径参数，必填）是要删除的会话的 ID。

`broadcastId`（请求体，必填）是用于实时同步的 UUID。

#### 示例请求

[inline-code-attrs-start title = 'DELETE 请求示例'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -X DELETE "https://fastcomments.com/comment-collab-chats/demo/conv789" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "broadcastId": "550e8400-e29b-41d4-a716-446655440001"
  }'
[inline-code-end]

#### 示例响应

[inline-code-attrs-start title = 'DELETE 响应示例'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success"
}
[inline-code-end]

### 速率限制

这些端点遵循标准的 FastComments API 速率限制。速率限制中间件按租户应用，以防止滥用，同时允许正常的使用模式。

### 错误响应

所有端点返回标准的 HTTP 状态代码。400 响应表示请求参数无效。401 响应表示身份验证失败。403 响应表示权限不足。404 响应表示未找到会话。429 响应表示超过速率限制。

错误响应包含带有详细信息的 JSON 正文：

[inline-code-attrs-start title = '错误响应示例'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "error",
  "message": "Conversation not found"
}
[inline-code-end]

### 使用情况跟踪

创建会话会增加您的 `conversationCreateCount` 使用指标。所有 WebSocket 同步活动都会增加 `pubSubMessageCount` 和 `pubSubBandwidth`。您可以在 FastComments 仪表板的使用分析中监控这些指标。

---
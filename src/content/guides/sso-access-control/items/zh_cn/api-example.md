在这里我们将演示如何调用 FastComments API 来设置访问控制。

在开始之前，请注意我们不必显式创建一个 `Group` 结构。组只是添加到 `Users` 和 `Pages` 的标识符。将组添加到用户或页面会自动“创建”该组。

首先，让我们创建两个用户，`User A` 和 `User B`，并将他们初始放入 `Group X`：

[inline-code-attrs-start title = '创建 User A 的 cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"id": "user-a",
	"username": "User A",
	"email": "usera@example.com",
    "groupIds": ["GROUP-X"]
}'
[inline-code-end]

[inline-code-attrs-start title = '创建 User B 的 cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"id": "user-b",
	"username": "User B",
	"email": "userb@example.com",
    "groupIds": ["GROUP-X"]
}'
[inline-code-end]

现在让我们创建一个 `Page`。我们将其命名为 `Confidential Page`，并且到目前为止这些用户都无法访问它，因为它将属于组 `CONFIDENTIAL`：

[inline-code-attrs-start title = '页面 POST cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"title": "Confidential Page",
	"url": "https://mysite.com/confidential",
	"urlId": "https://mysite.com/confidential",
	"accessibleByGroupIds": ["CONFIDENTIAL"]
}'
[inline-code-end]

Users A and B 当前 **不** 能访问该新页面。然而，由于他们在同一组 `GROUP-X` 中，他们可以互相 `@mention`。

让我们更新 `User B`，使其现在可以访问该页面：

[inline-code-attrs-start title = '更新 User B 的 cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "groupIds": ["GROUP-X", "CONFIDENTIAL"]
}'
[inline-code-end]

`User B` 现在属于两个组。我们的用户仍然可以互相 `@mention`，但只有 `User B` 可以查看我们的机密页面。

让 `User B` 仅能查看该机密页面：

[inline-code-attrs-start title = '更新 User B 的 cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "groupIds": ["CONFIDENTIAL"]
}'
[inline-code-end]

现在他们可以查看机密页面，但我们的任一用户都无法互相 `@mention`，因为他们属于不同的组。

然而，任何不在访问控制范围内的用户 **仍然可以访问我们的页面**。为防止这种情况，请确保没有 SSO 用户将其 `groupIds` 设置为 null。例如，让我们创建一个可以访问所有内容的 `User C`：

[inline-code-attrs-start title = '创建 User C 的 cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"id": "user-c",
	"username": "User C",
	"email": "userc@example.com",
    "groupIds": null
}'
[inline-code-end]

通过将 `groupIds` 设置为 null，我们表示他们不受访问控制的限制。

现在，让我们创建一个所有人都可以访问的页面：

[inline-code-attrs-start title = '页面 POST cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"title": "Public Page",
	"url": "https://mysite.com/public",
	"urlId": "https://mysite.com/public",
	"accessibleByGroupIds": null
}'
[inline-code-end]

通过将 `accessibleByGroupIds` 设置为 null，我们表示此 `Page` 不受访问控制的限制，两个用户都可以访问它。

这就完成了关于访问控制的 API 演练。
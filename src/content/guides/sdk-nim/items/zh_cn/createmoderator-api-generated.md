## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| createModeratorBody | CreateModeratorBody | 否 |  |

## 响应

返回: [`Option[CreateModeratorResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_moderator_response.nim)

## 示例

[inline-code-attrs-start title = 'createModerator 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
var body: CreateModeratorBody
body.username = "alice.moderator"
body.displayName = "Alice Moderator"
body.email = "alice@news-site.com"
body.enabled = true
body.roles = @["moderator"]
body.notes = ""

let (response, httpResponse) = client.createModerator(tenantId = "my-tenant-123", createModeratorBody = body)
if response.isSome:
  let created = response.get()
  echo "Created moderator ID: ", created.id
[inline-code-end]

---
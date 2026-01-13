## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| createModeratorBody | CreateModeratorBody | いいえ |  |

## レスポンス

返却値: [`Option[CreateModerator_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_moderator200response.nim)

## 例

[inline-code-attrs-start title = 'createModerator の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let createBody = CreateModeratorBody(
  email = "moderator@news-site.com",
  displayName = "News Moderator",
  permissions = @["approve_comments", "delete_comments"],
  isSuperAdmin = false
)

let (response, httpResponse) = client.createModerator(tenantId = "my-tenant-123", createModeratorBody = createBody)

if response.isSome:
  let moderator = response.get()
  echo "Created moderator: ", $moderator
[inline-code-end]

---
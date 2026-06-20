## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| createModeratorBody | CreateModeratorBody | 아니요 |  |

## 응답

반환: [`Option[CreateModeratorResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_moderator_response.nim)

## 예제

[inline-code-attrs-start title = 'createModerator 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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
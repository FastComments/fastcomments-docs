## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| createModeratorBody | CreateModeratorBody | Hayır |  |

## Yanıt

Döndürür: [`Option[CreateModeratorResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_moderator_response.nim)

## Örnek

[inline-code-attrs-start title = 'createModerator Örneği'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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
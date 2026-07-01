## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| createModeratorBody | CreateModeratorBody | Ne |  |

## Odgovor

Vraća: [`Option[CreateModeratorResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_moderator_response.nim)

## Primer

[inline-code-attrs-start title = 'createModerator Primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (moderatorRes, httpResp) = client.createModerator(tenantId = "my-tenant-123", createModeratorBody = CreateModeratorBody())
if moderatorRes.isSome:
  let moderator = moderatorRes.get()
[inline-code-end]
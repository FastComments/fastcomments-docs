## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Tak |  |
| createModeratorBody | CreateModeratorBody | Nie |  |

## Odpowiedź

Zwraca: [`Option[CreateModeratorResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_moderator_response.nim)

## Przykład

[inline-code-attrs-start title = 'Przykład createModerator'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (moderatorRes, httpResp) = client.createModerator(tenantId = "my-tenant-123", createModeratorBody = CreateModeratorBody())
if moderatorRes.isSome:
  let moderator = moderatorRes.get()
[inline-code-end]
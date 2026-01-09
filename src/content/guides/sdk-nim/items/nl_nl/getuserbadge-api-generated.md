## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Nee |  |

## Respons

Retourneert: [`Option[GetUserBadge_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_badge200response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'getUserBadge Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUserBadge(tenantId = "my-tenant-123", id = "")

if response.isSome:
  let badge = response.get()
  discard badge
[inline-code-end]

---
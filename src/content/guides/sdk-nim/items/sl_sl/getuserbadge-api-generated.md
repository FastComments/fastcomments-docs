## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Ne |  |

## Odgovor

Vrne: [`Option[GetUserBadge_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_badge200response.nim)

## Primer

[inline-code-attrs-start title = 'Primer getUserBadge'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUserBadge(tenantId = "my-tenant-123", id = "")

if response.isSome:
  let badge = response.get()
  discard badge
[inline-code-end]

---
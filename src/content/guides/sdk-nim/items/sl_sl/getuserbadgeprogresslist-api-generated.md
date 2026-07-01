## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| options | GetUserBadgeProgressListOptions | Ne |  |

## Odgovor

Vrne: [`Option[APIGetUserBadgeProgressListResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_get_user_badge_progress_list_response.nim)

## Primer

[inline-code-attrs-start title = 'Primer getUserBadgeProgressList'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUserBadgeProgressList(tenantId = "my-tenant-123", options = GetUserBadgeProgressListOptions())
if response.isSome:
  let badgeProgress = response.get()
[inline-code-end]
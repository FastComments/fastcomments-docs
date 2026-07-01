## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| sso | string = "" | No |  |

## Odgovor

Vraća: [`Option[GetBannedUsersCountResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_banned_users_count_response.nim)

## Primer

[inline-code-attrs-start title = 'Primer getCounts'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeCounts, httpRes) = client.getCounts(tenantId = "my-tenant-123", sso = "")
if maybeCounts.isSome:
  let counts = maybeCounts.get()
  echo counts
else:
  echo "No counts returned"
[inline-code-end]
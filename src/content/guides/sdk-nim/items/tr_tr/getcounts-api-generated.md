## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| sso | string = "" | No |  |

## Yanıt

Döndürür: [`Option[GetBannedUsersCountResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_banned_users_count_response.nim)

## Örnek

[inline-code-attrs-start title = 'getCounts Örneği'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeCounts, httpRes) = client.getCounts(tenantId = "my-tenant-123", sso = "")
if maybeCounts.isSome:
  let counts = maybeCounts.get()
  echo counts
else:
  echo "No counts returned"
[inline-code-end]
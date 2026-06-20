---
Sayfada daha önce yorum yapan ve şu anda çevrimiçi olmayan kişiler. displayName'e göre sıralanır.
/users/online tükendiğinde "Üyeler" bölümünü görüntülemek için bunu kullanın.
commenterName üzerinde imleçli sayfalama: sunucu kısmi {tenantId, urlId, commenterName}
indeksini afterName'den ileriye doğru $gt ile yürütür, $skip maliyeti yok.

## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| afterName | string | No |  |
| afterUserId | string | No |  |

## Yanıt

Döndürür: [`Option[PageUsersOfflineResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_offline_response.nim)

## Örnek

[inline-code-attrs-start title = 'getOfflineUsers Örneği'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getOfflineUsers(
  tenantId = "my-tenant-123",
  urlId = "news/article-how-to-code",
  afterName = "",
  afterUserId = ""
)

if response.isSome:
  let offlinePage = response.get()
  echo "Received offline users page"
  discard httpResponse.statusCode
[inline-code-end]

---
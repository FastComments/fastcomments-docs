Sayfada şu anda çevrimiçi OLMAYAN önceki yorumcular. displayName'e göre sıralanmıştır.
Bir "Üyeler" bölümü oluşturmak için /users/online tükendikten sonra bunu kullanın.
commenterName üzerinde cursor sayfalama: sunucu, {tenantId, urlId, commenterName} kısmi indeksini
afterName'den $gt ile ileriye doğru tarar, $skip maliyeti yoktur.

## Parametreler

| Ad | Tür | Zorunlu | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| urlId | string | Evet |  |
| options | GetOfflineUsersOptions | Hayır |  |

## Yanıt

Döndürür: [`Option[PageUsersOfflineResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_offline_response.nim)

## Örnek

[inline-code-attrs-start title = 'getOfflineUsers Örneği'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (offlineResp, httpResponse) = client.getOfflineUsers(
  tenantId = "my-tenant-123",
  urlId = "news/article-title",
  options = GetOfflineUsersOptions()
)
if offlineResp.isSome:
  let offline = offlineResp.get()
  echo offline)
[inline-code-end]
Past commenters on the page who are NOT currently online. Sorted by displayName.  
Sayfada daha önce yorum yapan, şu anda ONLINE olmayan yorumcular. displayName’ye göre sıralanır.  

Use this after exhausting /users/online to render a "Members" section.  
/users/online kaynağını tükettikten sonra bir "Members" bölümü oluşturmak için bunu kullanın.  

Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName} index from afterName forward via $gt, no $skip cost.  
commenterName üzerinde cursor sayfalama: sunucu, {tenantId, urlId, commenterName} kısmından afterName sonrası $gt ile ilerleyerek, $skip maliyeti olmadan indeksi yürütür.  

## Parametreler

| Ad | Tür | Gerekli | Açıklama |
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
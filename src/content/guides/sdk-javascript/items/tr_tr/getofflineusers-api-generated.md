Sayfadaki geçmişte yorum yapan ve şu anda çevrimiçi olmayan kullanıcılar. displayName'e göre sıralanır.
Bunu, /users/online tükendiğinde "Üyeler" bölümünü göstermek için kullanın.
commenterName üzerinde imleçli sayfalandırma: sunucu, kısmi {tenantId, urlId, commenterName} indeksinde afterName'den ileri doğru $gt ile yürür, $skip maliyeti yok.

## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| urlId | string | Evet |  |
| afterName | string | Hayır |  |
| afterUserId | string | Hayır |  |

## Yanıt

Döndürür: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersOfflineResponse.ts)

## Örnek

[inline-code-attrs-start title = 'getOfflineUsers Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant-9f4b2a6c';
const urlId: string = 'articles/product-launch-2025';

const offlinePageFirst: PageUsersOfflineResponse = await getOfflineUsers(tenantId, urlId);

const afterName: string = 'samantha.r';
const afterUserId: string = 'user_7d3a21f9';
const offlinePageNext: PageUsersOfflineResponse = await getOfflineUsers(tenantId, urlId, afterName, afterUserId);
[inline-code-end]

---
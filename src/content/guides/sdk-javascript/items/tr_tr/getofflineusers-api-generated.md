Sayfadaki, şu anda çevrimiçi olmayan önceki yorum yapan kullanıcılar. displayName'e göre sıralanır.
Bu, /users/online tükendikten sonra "Members" bölümünü göstermek için kullanılır.
commenterName üzerinde cursor sayfalaması: sunucu, kısmi {tenantId, urlId, commenterName} indeksinde afterName'den başlayıp ileriye doğru $gt ile yürür, $skip maliyeti yok.

## Parametreler

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| urlId | string | Evet |  |
| afterName | string | Hayır |  |
| afterUserId | string | Hayır |  |

## Yanıt

Döndürür: [`GetOfflineUsers200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetOfflineUsers200Response.ts)

## Örnek

[inline-code-attrs-start title = 'getOfflineUsers Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_prod_001';
const urlId: string = 'article-2026-06-15-how-ai-impacts';
const afterName: string = 'michael.smith';
const afterUserId: string = 'user_72b9';

const response: GetOfflineUsers200Response = await getOfflineUsers(tenantId, urlId, afterName, afterUserId);
[inline-code-end]

---
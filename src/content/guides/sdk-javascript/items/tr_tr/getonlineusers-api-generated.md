Bir sayfanın şu anda çevrimiçi olan izleyicileri: websocket oturumu şu anda sayfaya abone olan kişiler.
Döndürür anonCount + totalCount (oda genelindeki aboneler, saymadığımız anon görüntüleyiciler dahil).

## Parametreler

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| urlId | string | Evet |  |
| afterName | string | Hayır |  |
| afterUserId | string | Hayır |  |

## Yanıt

Döndürür: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersOnlineResponse.ts)

## Örnek

[inline-code-attrs-start title = 'getOnlineUsers Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8f3c2b7';
const urlId: string = 'article-2026-06-19-site-update';
const afterName: string = 'michael.hansen';
const afterUserId: string = 'user_00421';
const onlineUsers: PageUsersOnlineResponse = await getOnlineUsers(tenantId, urlId, afterName, afterUserId);
[inline-code-end]

---
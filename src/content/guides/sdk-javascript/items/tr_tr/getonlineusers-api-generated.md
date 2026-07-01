Şu anda çevrimiçi izleyiciler bir sayfada: websocket oturumu şu anda sayfaya abone olan kişiler.  
anonCount + totalCount değerini döndürür (odadaki tüm aboneler, saymadığımız anonim izleyiciler dahil).

## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| urlId | string | Evet |  |
| afterName | string | Hayır |  |
| afterUserId | string | Hayır |  |

## Yanıt

Döndürür: [`GetOnlineUsersResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetOnlineUsersResponse.ts)

## Örnek

[inline-code-attrs-start title = 'getOnlineUsers Örnek'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoOnlineUsers() {
  const tenantId: string = "tenant_12345";
  const urlId: string = "url_98765";

  // İsteğe bağlı sayfalama parametreleriyle
  const pagedResult: GetOnlineUsersResponse = await getOnlineUsers(
    tenantId,
    urlId,
    "alice_smith",
    "user_9"
  );

  // İsteğe bağlı sayfalama parametreleri olmadan
  const fullResult: GetOnlineUsersResponse = await getOnlineUsers(tenantId, urlId);
}
[inline-code-end]

---
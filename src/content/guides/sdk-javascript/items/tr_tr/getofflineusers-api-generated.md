---
Sayfadaki geçmiş yorumcular, şu anda çevrimiçi olmayanlar. displayName alanına göre sıralanır.  
Bu, /users/online uç noktasını tükettiğinizden sonra bir "Members" bölümü oluşturmak için kullanılır.  
commenterName üzerinde cursor sayfalama: sunucu {tenantId, urlId, commenterName} kısmı üzerinden afterName sonrası $gt ile ilerler, $skip maliyeti yok.

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| afterName | string | No |  |
| afterUserId | string | No |  |

## Response

Döndürür: [`GetOfflineUsersResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetOfflineUsersResponse.ts)

## Örnek

[inline-code-attrs-start title = 'getOfflineUsers Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchOfflineUsers(): Promise<void> {
    const tenantId: string = "tenant_12345";
    const urlId: string = "thread_9876";
    const afterName: string = "Jane Smith";
    const afterUserId: string = "user_7f9b3c";

    const offlineUsers: GetOfflineUsersResponse = await getOfflineUsers(
        tenantId,
        urlId,
        afterName,
        afterUserId
    );

    console.log(offlineUsers);
}
[inline-code-end]

---
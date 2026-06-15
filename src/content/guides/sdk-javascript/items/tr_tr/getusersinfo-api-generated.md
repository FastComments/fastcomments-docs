---
Bir kiracı için toplu kullanıcı bilgisi. Belirtilen userIds'e göre User / SSOUser'dan görüntüleme bilgilerini döndürür.
Yorum widget'ı tarafından, presence olayı ile yeni görünen kullanıcıları zenginleştirmek için kullanılır.
Sayfa bağlamı yok: gizlilik tutarlı şekilde uygulanır (özel profiller maskelenir).

## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| ids | string | Evet |  |

## Yanıt

Döndürür: [`GetUsersInfo200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUsersInfo200Response.ts)

## Örnek

[inline-code-attrs-start title = 'getUsersInfo Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-007';
const userIdsList: string[] = ['user_12a', 'user_34b', 'user_56c'];
const separator: string | undefined = undefined; // isteğe bağlı; undefined ise varsayılan olarak virgül kullanılır
const ids: string = userIdsList.join(separator ?? ',');
const usersInfo: GetUsersInfo200Response = await getUsersInfo(tenantId, ids);
[inline-code-end]

---
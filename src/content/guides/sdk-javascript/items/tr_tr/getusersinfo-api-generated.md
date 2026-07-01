Bir kiracı için toplu kullanıcı bilgisi. userId'ler verildiğinde, User / SSOUser'dan görüntüleme bilgisi döndürür.  
Yorum widget'ı tarafından, bir varlık etkinliğiyle yeni ortaya çıkan kullanıcıları zenginleştirmek için kullanılır.  
Sayfa bağlamı yok: gizlilik tutarlı bir şekilde uygulanır (özel profiller maskelelenir).

## Parametreler

| İsim | Tür | Zorunlu | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| ids | string | Evet |  |

## Yanıt

Döndürür: [`GetUsersInfoResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUsersInfoResponse.ts)

## Örnek

[inline-code-attrs-start title = 'getUsersInfo Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-tenant";
const ids: string = "user-1001,user-1002";

const usersInfo: GetUsersInfoResponse = await getUsersInfo(tenantId, ids);

// Yanıt içindeki isteğe bağlı alanlar tanımsız olabilir
const firstUser: PageUserEntry | undefined = usersInfo?.users?.[0];
[inline-code-end]

---
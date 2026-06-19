Bir kiracı için toplu kullanıcı bilgisi. userIds verildiğinde, User / SSOUser'dan görüntü bilgilerini döndürür.
Yorum bileşeni tarafından, presence olayı aracılığıyla yeni ortaya çıkan kullanıcıları zenginleştirmek için kullanılır.
Sayfa bağlamı yok: gizlilik tutarlı şekilde uygulanır (özel profiller maskelenir).

## Parametreler

| İsim | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| ids | string | Evet |  |

## Yanıt

Döndürür: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersInfoResponse.ts)

## Örnek

[inline-code-attrs-start title = 'getUsersInfo Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_78f9';
const ids: string = 'user_10234,user_10235,user_10236';
const usersInfo: PageUsersInfoResponse = await getUsersInfo(tenantId, ids);
// getUsersInfo sadece tenantId ve ids'yi gerektirir; isteğe bağlı parametreler burada uygulanmaz.
[inline-code-end]

---
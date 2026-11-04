Bir kiracı (tenant) için toplu kullanıcı bilgisi. Verilen userIds değerlerine göre, User / SSOUser kaynaklarından görüntüleme bilgilerini döndürür.
Yorum widget'ı tarafından, bir varlık (presence) olayı aracılığıyla yeni görünen kullanıcıları zenginleştirmek için kullanılır.
Sayfa bağlamı yoktur: gizlilik tek tip şekilde uygulanır (özel profiller maskelenir).

## Parametreler

| Ad | Tür | Zorunlu | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| ids | string | Evet |  |

## Yanıt

Döndürülen değer: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersInfoResponse.ts)

## Örnek

[inline-code-attrs-start title = 'getUsersInfo Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchUsersInfo(): Promise<void> {
  const tenantId: string = "tenant_12345";
  const ids: string = "user_001,user_002";
  const response: PageUsersInfoResponse = await getUsersInfo(tenantId, ids);
  console.log(response);
}
fetchUsersInfo();
[inline-code-end]
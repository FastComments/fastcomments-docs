Bulk user info for a tenant. Given userIds, return display info from User / SSOUser.  
Bir kiracı için toplu kullanıcı bilgisi. userIds verildiğinde, User / SSOUser'dan görüntüleme bilgilerini döndürür.

Used by the comment widget to enrich users that just appeared via a presence event.  
Yorum widget'ı tarafından, bir varlık olayıyla yeni ortaya çıkan kullanıcıları zenginleştirmek için kullanılır.

No page context: privacy is enforced uniformly (private profiles are masked).  
Sayfa bağlamı yok: gizlilik tutarlı bir şekilde uygulanır (özel profiller maskeleme yapılır).

## Parameters

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| ids | string | Yes |  |

## Response

Döndürür: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersInfoResponse.ts)

## Example

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

---
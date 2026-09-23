Past commenters on the page who are NOT currently online. Sorted by displayName.  
Sayfada daha önce yorum yapmış ancak şu anda ONLINE olmayan yorumcular. displayName'e göre sıralanır.

Use this after exhausting /users/online to render a "Members" section.  
/users/online'ı tüketip ardından bir "Members" bölümü oluşturmak için bunu kullanın.

Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName}  
index from afterName forward via $gt, no $skip cost.  
commenterName üzerinde imleç sayfalama: sunucu {tenantId, urlId, commenterName} kısmını yürütür, afterName'den itibaren $gt ile ileriye doğru indeksler, $skip maliyeti yok.

## Parameters

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| afterName | string | No |  |
| afterUserId | string | No |  |

## Response

Döndürür: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersOfflineResponse.ts)

## Example

[inline-code-attrs-start title = 'getOfflineUsers Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchOfflineUsers(): Promise<void> {
  const tenantId: string = "tenant_12345";
  const urlId: string = "page_9876";
  const afterName: string = "John Doe";
  const afterUserId: string = "user_abc123";

  const offlineResponse: PageUsersOfflineResponse = await getOfflineUsers(
    tenantId,
    urlId,
    afterName,
    afterUserId
  );

  console.log(offlineResponse);
}
[inline-code-end]
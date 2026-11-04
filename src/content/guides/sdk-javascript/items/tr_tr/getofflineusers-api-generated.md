Sayfada şu anda çevrimiçi OLMAYAN önceki yorumcular. displayName'e göre sıralı.
"Bir "Üyeler" bölümü oluşturmak için /users/online kaynağını tükettikten sonra bunu kullanın.
commenterName üzerinde imleç tabanlı sayfalama: sunucu, {tenantId, urlId, commenterName} kısmi indeksini
afterName'den $gt ile ileri doğru gezinir, $skip maliyeti yoktur.

## Parametreler

| Ad | Tür | Zorunlu | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| urlId | string | Evet |  |
| afterName | string | Hayır |  |
| afterUserId | string | Hayır |  |

## Yanıt

Döndürür: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersOfflineResponse.ts)

## Örnek

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
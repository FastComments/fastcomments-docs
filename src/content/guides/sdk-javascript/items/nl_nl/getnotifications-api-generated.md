## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| userId | string | Nee |  |
| urlId | string | Nee |  |
| fromCommentId | string | Nee |  |
| viewed | boolean | Nee |  |
| type | string | Nee |  |
| skip | number | Nee |  |

## Respons

Retourneert: [`GetNotifications200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetNotifications200Response.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getNotifications Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_8f3b1a2c';
  const userId: string = 'user_42';
  const urlId: string = 'https://news.example.com/articles/2026/01/11/comment-thread';
  const fromCommentId: string = 'cmt_9a7b';
  const viewed: boolean = false;
  const type: string = 'mention';
  const skip: number = 0;
  const response: GetNotifications200Response = await getNotifications(tenantId, userId, urlId, fromCommentId, viewed, type, skip);
  console.log(response);
})();
[inline-code-end]

---
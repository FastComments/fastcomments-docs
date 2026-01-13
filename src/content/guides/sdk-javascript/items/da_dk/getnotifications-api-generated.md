---
## Parametre

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| userId | string | Nej |  |
| urlId | string | Nej |  |
| fromCommentId | string | Nej |  |
| viewed | boolean | Nej |  |
| type | string | Nej |  |
| skip | number | Nej |  |

## Svar

Returnerer: [`GetNotifications200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetNotifications200Response.ts)

## Eksempel

[inline-code-attrs-start title = 'getNotifications Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|---------|-------------|
| tenantId | string | Ja |  |
| userId | string | Nee |  |
| urlId | string | Nee |  |
| fromCommentId | string | Nee |  |
| viewed | boolean | Nee |  |
| type | string | Nee |  |

## Respons

Retourneert: [`GetNotificationCountResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetNotificationCountResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getNotificationCount Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f3b2a';
const userId: string = 'user_7421';
const urlId: string = 'https://news.example.com/articles/2026/06/19/ai-update';
const fromCommentId: string = 'cmt_5a1d2f';
const viewed: boolean = false;
const type: string = 'mention';

const notificationCount: GetNotificationCountResponse = await getNotificationCount(
  tenantId,
  userId,
  urlId,
  fromCommentId,
  viewed,
  type
);
[inline-code-end]

---
---
## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| userId | string | Nej |  |
| urlId | string | Nej |  |
| fromCommentId | string | Nej |  |
| viewed | boolean | Nej |  |
| type | string | Nej |  |

## Svar

Returnerer: [`GetNotificationCountResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetNotificationCountResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på getNotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
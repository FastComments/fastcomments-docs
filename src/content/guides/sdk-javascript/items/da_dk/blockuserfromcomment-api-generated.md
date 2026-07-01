## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| blockFromCommentParams | BlockFromCommentParams | Ja |  |
| userId | string | Nej |  |
| anonUserId | string | Nej |  |

## Svar

Returnerer: [`BlockUserFromCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BlockUserFromCommentResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'blockUserFromComment Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_42';
const commentId: string = 'cmt_20231101';

const blockParams: BlockFromCommentParams = {
  reason: 'spam',
  blockDurationHours: 24,
};

const userId: string = 'user_123'; // valgfri parameter

const response: BlockUserFromCommentResponse = await blockUserFromComment(
  tenantId,
  commentId,
  blockParams,
  userId
);
[inline-code-end]

---
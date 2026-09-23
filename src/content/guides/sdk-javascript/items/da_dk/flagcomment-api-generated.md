## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| userId | string | No |  |
| anonUserId | string | No |  |

## Svar

Returnerer: [`FlagCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'flagComment Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp";
  const commentId: string = "comment-20230915-001";
  const userId: string = "user-42";
  const anonUserId: string = "anon-abc123";

  const response: FlagCommentResponse = await flagComment(tenantId, commentId, userId, anonUserId);
  console.log(response);
})();
[inline-code-end]
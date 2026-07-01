## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| userId | string | No |  |
| anonUserId | string | No |  |

## Odgovor

Vrne: [`FlagCommentResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentResponse1.ts)

## Primer

[inline-code-attrs-start title = 'flagComment Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_2023";
  const commentId: string = "comment_5678";
  const userId: string = "user_1234";
  const anonUserId: string = "anon_4321";

  const result: FlagCommentResponse1 = await flagComment(tenantId, commentId, userId);
  const anonResult: FlagCommentResponse1 = await flagComment(tenantId, commentId, undefined, anonUserId);
})();
[inline-code-end]
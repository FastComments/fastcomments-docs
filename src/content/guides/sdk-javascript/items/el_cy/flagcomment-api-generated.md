## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|------------|-----------|
| tenantId | string | Ναι |  |
| id | string | Ναι |  |
| userId | string | Όχι |  |
| anonUserId | string | Όχι |  |

## Απόκριση

Επιστρέφει: [`FlagCommentResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentResponse1.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα flagComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|-----------|-----------|
| tenantId | string | Ναι |  |
| id | string | Ναι |  |
| userId | string | Όχι |  |
| anonUserId | string | Όχι |  |

## Απόκριση

Επιστρέφει: [`UnFlagCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UnFlagCommentResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα unFlagComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp";
const commentId: string = "cmt_123456";
const userId: string = "usr_98765";

const result: UnFlagCommentResponse = await unFlagComment(tenantId, commentId, userId);

const anonCommentId: string = "cmt_123457";
const anonUserId: string = "anon_abc123";

const anonResult: UnFlagCommentResponse = await unFlagComment(tenantId, anonCommentId, undefined, anonUserId);
[inline-code-end]
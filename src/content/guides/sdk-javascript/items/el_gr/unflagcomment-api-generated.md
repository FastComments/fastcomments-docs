## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| id | string | Ναι |  |
| userId | string | Όχι |  |
| anonUserId | string | Όχι |  |

## Απόκριση

Επιστρέφει: [`FlagCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα unFlagComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-001';
const id: string = 'cmt-9b8f7d6a5';
const userId: string = 'user-42a7c9e1';

const result: FlagCommentResponse = await unFlagComment(tenantId, id, userId);
[inline-code-end]

---
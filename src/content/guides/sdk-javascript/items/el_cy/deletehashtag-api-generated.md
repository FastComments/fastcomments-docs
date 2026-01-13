## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tag | string | Ναι |  |
| tenantId | string | Όχι |  |
| deleteHashTagRequest | DeleteHashTagRequest | Όχι |  |

## Απόκριση

Επιστρέφει: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα deleteHashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tag: string = 'breaking-news';
const tenantId: string = 'tenant_42';
const deleteReq: DeleteHashTagRequest = { removedBy: 'moderator_jane', reason: 'off-topic for this community', deleteAssociatedComments: true } as DeleteHashTagRequest;
const result: FlagCommentPublic200Response = await deleteHashTag(tag, tenantId, deleteReq);
[inline-code-end]
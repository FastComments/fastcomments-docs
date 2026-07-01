## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|------------|------------|
| commentId | string | Ναι |  |
| tenantId | string | Όχι |  |
| sso | string | Όχι |  |

## Απάντηση

Επιστρέφει: [`GetCommentChildrenResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentChildrenResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'getCommentChildren Παράδειγμα'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const commentId: string = "cmt_12345";
const tenantId: string = "tenant_xyz";
const sso: string = "sso_987654";

const fullResponse: GetCommentChildrenResponse = await getCommentChildren(commentId, tenantId, sso);
const minimalResponse: GetCommentChildrenResponse = await getCommentChildren(commentId);
[inline-code-end]

---
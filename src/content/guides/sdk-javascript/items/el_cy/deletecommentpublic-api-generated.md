---
## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| commentId | string | Ναι |  |
| broadcastId | string | Ναι |  |
| editKey | string | Όχι |  |
| sso | string | Όχι |  |

## Απόκριση

Επιστρέφει: [`DeleteCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteCommentPublic200Response.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα deleteCommentPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_42';
const commentId: string = 'c0mment-9f8b7a6';
const broadcastId: string = 'site_homepage_2026-06-15';
const editKey: string = 'ek_3b7a1f59-4d2c-11eb-8dcd-0242ac130003';
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.fakePayload.signature';

const result: DeleteCommentPublic200Response = await deleteCommentPublic(tenantId, commentId, broadcastId, editKey, sso);
[inline-code-end]

---
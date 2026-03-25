## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| postId | string | Ναι |  |
| reactBodyParams | ReactBodyParams | Ναι |  |
| isUndo | boolean | Όχι |  |
| broadcastId | string | Όχι |  |
| sso | string | Όχι |  |

## Απόκριση

Επιστρέφει: [`ReactFeedPostPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ReactFeedPostPublic200Response.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα reactFeedPostPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'd290f1ee-6c54-4b01-90e6-d701748f0851';
const postId: string = 'c1a2b3d4-5678-90ab-cdef-1234567890ab';
const reactBodyParams: ReactBodyParams = { reaction: 'like', emoji: '👍', source: 'web' };
const isUndo: boolean = false;
const broadcastId: string = 'broadcast-2026-03-25-001';
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIn0.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c';
const result: ReactFeedPostPublic200Response = await reactFeedPostPublic(tenantId, postId, reactBodyParams, isUndo, broadcastId, sso);
[inline-code-end]
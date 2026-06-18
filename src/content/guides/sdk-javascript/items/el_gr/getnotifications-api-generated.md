---
## Παράμετροι

| Name | Type | Απαιτείται | Περιγραφή |
|------|------|------------|-----------|
| tenantId | string | Ναι |  |
| userId | string | Όχι |  |
| urlId | string | Όχι |  |
| fromCommentId | string | Όχι |  |
| viewed | boolean | Όχι |  |
| type | string | Όχι |  |
| skip | number | Όχι |  |

## Απάντηση

Επιστρέφει: [`GetNotifications200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetNotifications200Response.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getNotifications'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f8b7c6a';
const userId: string = 'user_5a4b3c2d';
const urlId: string = 'post_84f2a1b9';
const fromCommentId: string = 'cmt_0a1b2c3d';
const viewed: boolean = false;
const type: string = 'reply';
const skip: number = 0;

const notifications: GetNotifications200Response = await getNotifications(
  tenantId,
  userId,
  urlId,
  fromCommentId,
  viewed,
  type,
  skip
);
[inline-code-end]

---
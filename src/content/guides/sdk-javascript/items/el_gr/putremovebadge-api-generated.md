## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| badgeId | string | Yes |  |
| userId | string | No |  |
| commentId | string | No |  |
| broadcastId | string | No |  |
| tenantId | string | No |  |
| sso | string | No |  |

## Απάντηση

Επιστρέφει: [`PutRemoveBadgeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PutRemoveBadgeResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα putRemoveBadge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const badgeId: string = "badge-12345";
const userId: string = "user-9876";
const commentId: string = "comment-5555";
const broadcastId: string = "broadcast-001";

const result: PutRemoveBadgeResponse = await putRemoveBadge(
  badgeId,
  userId,
  commentId,
  broadcastId
);
[inline-code-end]
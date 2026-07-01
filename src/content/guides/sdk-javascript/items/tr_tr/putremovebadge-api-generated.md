## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| badgeId | string | Evet |  |
| userId | string | Hayır |  |
| commentId | string | Hayır |  |
| broadcastId | string | Hayır |  |
| tenantId | string | Hayır |  |
| sso | string | Hayır |  |

## Yanıt

Döndürür: [`PutRemoveBadgeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PutRemoveBadgeResponse.ts)

## Örnek

[inline-code-attrs-start title = 'putRemoveBadge Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---
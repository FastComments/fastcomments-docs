## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| userId | string | いいえ |  |
| badgeId | string | いいえ |  |
| type | number | いいえ |  |
| displayedOnComments | boolean | いいえ |  |
| limit | number | いいえ |  |
| skip | number | いいえ |  |

## レスポンス

返却: [`GetUserBadges200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserBadges200Response.ts)

## 例

[inline-code-attrs-start title = 'getUserBadges の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_01';
const userId: string = 'user_5f4d3c2a';
const badgeId: string = 'badge_top_contributor';
const type: number = 1;
const displayedOnComments: boolean = true;
const limit: number = 50;
const skip: number = 0;

const result: GetUserBadges200Response = await getUserBadges(tenantId, userId, badgeId, type, displayedOnComments, limit, skip);
[inline-code-end]

---
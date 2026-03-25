## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| userId | string | 否 |  |
| badgeId | string | 否 |  |
| type | number | 否 |  |
| displayedOnComments | boolean | 否 |  |
| limit | number | 否 |  |
| skip | number | 否 |  |

## 响应

返回： [`GetUserBadges200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserBadges200Response.ts)

## 示例

[inline-code-attrs-start title = 'getUserBadges 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
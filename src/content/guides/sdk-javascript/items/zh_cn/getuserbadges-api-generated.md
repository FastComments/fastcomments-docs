## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| userId | string | 否 |  |
| badgeId | string | 否 |  |
| type | number | 否 |  |
| displayedOnComments | boolean | 否 |  |
| limit | number | 否 |  |
| skip | number | 否 |  |

## 响应

返回：[`APIGetUserBadgesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIGetUserBadgesResponse.ts)

## 示例

[inline-code-attrs-start title = 'getUserBadges 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7a1c9f2b";
const userId: string = "user_4b2d1e9a";
const badgeId: string = "badge_gold_01";
const type: number = 2;
const displayedOnComments: boolean = true;
const limit: number = 25;
const skip: number = 0;

const response: APIGetUserBadgesResponse = await getUserBadges(
  tenantId,
  userId,
  badgeId,
  type,
  displayedOnComments,
  limit,
  skip
);
[inline-code-end]
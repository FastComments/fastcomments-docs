## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| userId | string | 아니오 |  |
| badgeId | string | 아니오 |  |
| type | number | 아니오 |  |
| displayedOnComments | boolean | 아니오 |  |
| limit | number | 아니오 |  |
| skip | number | 아니오 |  |

## 응답

반환: [`GetUserBadges200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserBadges200Response.ts)

## 예제

[inline-code-attrs-start title = 'getUserBadges 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7f9a12';
const userId: string = 'user_42b7';
const badgeId: string = 'badge_top_contributor';
const type: number = 2;
const displayedOnComments: boolean = true;
const limit: number = 25;
const skip: number = 0;
const badges: GetUserBadges200Response = await getUserBadges(tenantId, userId, badgeId, type, displayedOnComments, limit, skip);
[inline-code-end]

---
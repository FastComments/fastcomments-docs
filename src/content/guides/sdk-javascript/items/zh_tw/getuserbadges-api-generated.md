## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|------|------|
| tenantId | string | 是 |  |
| userId | string | 否 |  |
| badgeId | string | 否 |  |
| type | number | 否 |  |
| displayedOnComments | boolean | 否 |  |
| limit | number | 否 |  |
| skip | number | 否 |  |

## 回應

回傳：[`APIGetUserBadgesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIGetUserBadgesResponse.ts)

## 範例

[inline-code-attrs-start title = 'getUserBadges 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp";

  // 提供所有參數
  const fullResponse: APIGetUserBadgesResponse = await getUserBadges(
    tenantId,
    "user-42",
    "badge-premium",
    1,
    true,
    10,
    0
  );

  // 只提供必填參數與一個可選參數 (limit)
  const limitedResponse: APIGetUserBadgesResponse = await getUserBadges(
    tenantId,
    undefined,
    undefined,
    undefined,
    undefined,
    5
  );

  console.log(fullResponse, limitedResponse);
})();
[inline-code-end]
## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| userId | string | 아니오 |  |
| badgeId | string | 아니오 |  |
| type | number | 아니오 |  |
| displayedOnComments | boolean | 아니오 |  |
| limit | number | 아니오 |  |
| skip | number | 아니오 |  |

## 응답

반환: [`APIGetUserBadgesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIGetUserBadgesResponse.ts)

## 예시

[inline-code-attrs-start title = 'getUserBadges 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp";

  // 모든 매개변수가 제공됨
  const fullResponse: APIGetUserBadgesResponse = await getUserBadges(
    tenantId,
    "user-42",
    "badge-premium",
    1,
    true,
    10,
    0
  );

  // 필수 매개변수와 하나의 선택 매개변수(limit)만 제공됨
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
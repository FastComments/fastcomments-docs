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

반환: [`GetUserBadgesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserBadgesResponse.ts)

## 예제

[inline-code-attrs-start title = 'getUserBadges 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function example() {
  const tenantId: string = "tenant-01";
  const userId: string = "user-42";
  const badgeId: string = "badge-gold";
  const type: number = 1;
  const displayedOnComments: boolean = true;
  const limit: number = 10;
  const skip: number = 5;

  const fullResult: GetUserBadgesResponse = await getUserBadges(
    tenantId,
    userId,
    badgeId,
    type,
    displayedOnComments,
    limit,
    skip
  );

  const minimalResult: GetUserBadgesResponse = await getUserBadges(tenantId);
}
example();
[inline-code-end]
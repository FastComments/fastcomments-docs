---
## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| userId | string | Yes |  |

## 응답

반환: [`APIGetUserBadgeProgressResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIGetUserBadgeProgressResponse.ts)

## 예제

[inline-code-attrs-start title = 'getUserBadgeProgressByUserId 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = '9f3a2c7e-1b5d-4f8a-9a2e-123456789abc';
const userId: string = 'd4e8f0a1-6b3c-4d2e-8f1b-9876543210fe';
const result: APIGetUserBadgeProgressResponse = await getUserBadgeProgressByUserId(tenantId, userId);
const status: APIStatus | undefined = (result as unknown as { status?: APIStatus }).status;
const badgeProgress: UserBadgeProgress[] | undefined = (result as unknown as { badgeProgress?: UserBadgeProgress[] }).badgeProgress;
[inline-code-end]

---
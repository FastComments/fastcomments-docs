## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |

## 응답

반환: [`GetUserBadgeProgressById200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserBadgeProgressById200Response.ts)

## 예제

[inline-code-attrs-start title = 'getUserBadgeProgressById 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'fastcomments-tenant-241';
const badgeId: string = 'user-78b3d-badge-3';
const response: GetUserBadgeProgressById200Response = await getUserBadgeProgressById(tenantId, badgeId);
const progress: UserBadgeProgress | undefined = (response as unknown as { progress?: UserBadgeProgress }).progress;
const percentComplete: number | undefined = progress?.percentage;
console.log('Badge progress percent complete:', percentComplete);
[inline-code-end]

---
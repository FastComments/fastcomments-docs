## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## 응답

반환: [`APIGetUserBadgeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIGetUserBadgeResponse.ts)

## 예시

[inline-code-attrs-start title = 'getUserBadge 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoGetUserBadge() {
    const tenantId: string = "tenant-9f8b7c6d";
    const userId: string = "user-123456";
    const badgeResponse: APIGetUserBadgeResponse = await getUserBadge(tenantId, userId);
}
[inline-code-end]
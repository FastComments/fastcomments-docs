## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |

## 응답

반환: [`APIGetUserBadgeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIGetUserBadgeResponse.ts)

## 예제

[inline-code-attrs-start title = 'getUserBadge 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-72a1';
const id: string = 'badge_5d8f3c9';
const response: APIGetUserBadgeResponse = await getUserBadge(tenantId, id);
const status: APIStatus = response.status;
const badgeTitle: string | undefined = response.userBadge?.title;
[inline-code-end]

---
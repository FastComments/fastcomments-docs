## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |

## Одговор

Враћа: [`APIGetUserBadgeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIGetUserBadgeResponse.ts)

## Примјер

[inline-code-attrs-start title = 'getUserBadge Примјер'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-72a1';
const id: string = 'badge_5d8f3c9';
const response: APIGetUserBadgeResponse = await getUserBadge(tenantId, id);
const status: APIStatus = response.status;
const badgeTitle: string | undefined = response.userBadge?.title;
[inline-code-end]

---
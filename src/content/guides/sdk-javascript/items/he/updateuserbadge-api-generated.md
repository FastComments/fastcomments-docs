## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| id | string | כן |  |
| updateUserBadgeParams | UpdateUserBadgeParams | כן |  |

## תגובה

מחזיר: [`UpdateUserBadge200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserBadge200Response.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמת updateUserBadge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9a8b7c";
const id: string = "badge_a1b2c3";
const updateUserBadgeParams: UpdateUserBadgeParams = {
  title: "Community Helper",
  description: "Awarded for providing 50 helpful answers",
  iconUrl: "https://cdn.fastcomments.com/badges/community-helper.png",
  isActive: true,
  expiryDate: undefined
};
const result: UpdateUserBadge200Response = await updateUserBadge(tenantId, id, updateUserBadgeParams);
[inline-code-end]

---
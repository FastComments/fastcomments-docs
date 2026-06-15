## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| createUserBadgeParams | CreateUserBadgeParams | כן |  |

## תגובה

מחזיר: [`CreateUserBadge200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateUserBadge200Response.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-createUserBadge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7f4b2a";
const createUserBadgeParams: CreateUserBadgeParams = {
  code: "top_contributor",
  title: "Top Contributor",
  description: "Awarded for 100 high-quality comments",
  iconUrl: "https://cdn.fastcomments.com/badges/top_contributor.svg",
  isActive: true,
  criteria: { commentsRequired: 100 },
  customConfig: { displayOnProfile: true } // הדגמת פרמטר אופציונלי
};
const result: CreateUserBadge200Response = await createUserBadge(tenantId, createUserBadgeParams);
[inline-code-end]

---
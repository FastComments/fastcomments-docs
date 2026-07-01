## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| createUserBadgeParams | CreateUserBadgeParams | כן |  |

## תגובה

מחזיר: [`CreateUserBadgeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateUserBadgeResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'createUserBadge דוגמה'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "company-42";

const badgeParams: CreateUserBadgeParams = {
  name: "Community Champion",
  iconUrl: "https://assets.example.com/badges/champion.png",
  // תיאור הוא אופציונלי ווסר כאן
};

const result: CreateUserBadgeResponse = await createUserBadge(tenantId, badgeParams);
[inline-code-end]

---
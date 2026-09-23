---
מזהה את האישור בשימוש: השוכר שאליו הוא שייך, ובמקרה של אסימוני OAuth, המשתמש שהאשר אותו.  
אינטגרציות משתמשות בזה כדי לבדוק חיבור ולתייג אותו.

## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |

## תגובה

מחזיר: [`GetMeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetMeResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getMe'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp-tenant";
  const me: GetMeResponse = await getMe(tenantId);
  const authType: MeAuthType = me.auth.type;
  const scopes: OAuthScope[] = me.auth.scopes ?? [];
  const status: APIStatus = me.status;
})();
[inline-code-end]

---
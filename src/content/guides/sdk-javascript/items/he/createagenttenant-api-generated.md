יוצר חשבון ניסיון חדש עבור סוכן AI ללא הרשמה של בן אדם. אין צורך במפתח API כדי לקרוא לפונקציה זו.

התשובה מכילה את מזהה הטננט, מפתח API שעובד מיד מול ה-REST API ושרת ה-MCP, וכתובת URL לתביעה. מסרו את כתובת ה-URL לתביעה לבן האדם שאתם עובדים עבורו: פתיחתה בזמן שמחוברים ל-FastComments מצמידה את החשבון אליו. חשבונות שלא נתבעו, והמפתחות שלהם, נמחקים 72 שעות לאחר היצירה. עד לתביעה, לחשבון יש את מגבלות הניסיון הרגילות.

## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| createAgentTenantBody | CreateAgentTenantBody | Yes |  |

## תשובה

מחזיר: [`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateAgentTenantResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-createAgentTenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantBody: CreateAgentTenantBody = {
  name: "Acme Corp",
  domain: "acme.example.com",
  contactEmail: "admin@acme.example.com", // optional
  planId: 3 // optional
};

const result: CreateAgentTenantResponse = await createAgentTenant(tenantBody);
[inline-code-end]
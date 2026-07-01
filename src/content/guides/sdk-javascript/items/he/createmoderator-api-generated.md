## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createModeratorBody | CreateModeratorBody | Yes |  |

## תגובה

מחזיר: [`CreateModeratorResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateModeratorResponse1.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמת createModerator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample() {
  const tenantId: string = "tenant_9876";
  const moderatorPayload: CreateModeratorBody = {
    name: "Alice Johnson",
    email: "alice.johnson@example.com"
    // שדות אופציונליים כמו תיאור נשמטו
  };
  const result: CreateModeratorResponse1 = await createModerator(tenantId, moderatorPayload);
  console.log(result);
}

runExample();
[inline-code-end]
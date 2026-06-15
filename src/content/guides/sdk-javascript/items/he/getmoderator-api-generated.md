## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| id | string | כן |  |

## תגובה

מחזיר: [`GetModerator200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModerator200Response.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getModerator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-media-58';
const id: string = 'mod-82f3b9c1';
const moderatorResponse: GetModerator200Response = await getModerator(tenantId, id);
[inline-code-end]

---
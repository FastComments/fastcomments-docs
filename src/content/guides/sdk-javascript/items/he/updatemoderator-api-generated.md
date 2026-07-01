## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| id | string | כן |  |
| updateModeratorBody | UpdateModeratorBody | כן |  |

## תגובה

מחזיר: [`UpdateModeratorResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateModeratorResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמת updateModerator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoUpdateModerator(): Promise<void> {
    const tenantId: string = "tenant_42abc";
    const moderatorId: string = "moderator_8f9e";
    const updateBody: UpdateModeratorBody = {
        isActive: true,
        role: "admin",
        // שדה אופציונלי
        notes: "Promoted to senior moderator"
    };
    const result: UpdateModeratorResponse = await updateModerator(tenantId, moderatorId, updateBody);
    console.log(result);
}

demoUpdateModerator();
[inline-code-end]
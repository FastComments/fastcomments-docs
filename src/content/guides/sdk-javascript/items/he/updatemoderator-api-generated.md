## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| id | string | כן |  |
| updateModeratorBody | UpdateModeratorBody | כן |  |

## תגובה

מחזיר: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-updateModerator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = '4f8a9c2e-3b6d-4d9e-8c2f-1a2b3c4d5e6f';
const id: string = 'mod_92a7c4';
const updateModeratorBodyMinimal: UpdateModeratorBody = { displayName: 'Sophia Patel' };
const updateModeratorBodyWithOptional: UpdateModeratorBody = {
  displayName: 'Sophia Patel',
  email: 'sophia.patel@newsroom.example',
  permissions: ['remove_comments', 'ban_user'],
  notifyOnFlag: true // פרמטר אופציונלי לדוגמה
};
const result: FlagCommentPublic200Response = await updateModerator(tenantId, id, updateModeratorBodyWithOptional);
[inline-code-end]

---
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
const tenantId: string = 'acme-tenant-81';
const id: string = 'mod_7f3a2b';
const updateModeratorBody: UpdateModeratorBody = {
  email: 'j.reyes@acme-corp.com',
  displayName: 'Jordan Reyes',
  roles: ['moderator', 'content_reviewer'],
  active: true,
  notes: 'Promoted to senior moderator; monitor flagged content weekly'
};
const result: FlagCommentPublic200Response = await updateModerator(tenantId, id, updateModeratorBody);
[inline-code-end]
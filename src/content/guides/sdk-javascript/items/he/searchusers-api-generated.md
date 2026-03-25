## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| urlId | string | כן |  |
| usernameStartsWith | string | לא |  |
| mentionGroupIds | Array<string> | לא |  |
| sso | string | לא |  |
| searchSection | SearchUsersSearchSectionEnum | לא |  |

## תגובה

מוחזר: [`SearchUsers200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SearchUsers200Response.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-searchUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8392';
const urlId: string = 'articles/2026/03/25/fastcomments-release';
const usernameStartsWith: string = 'jo';
const mentionGroupIds: Array<string> = ['editors', 'senior-writers'];
const sso: string = 'sso_jwt_eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9';
const searchSection: SearchUsersSearchSectionEnum = SearchUsersSearchSectionEnum.ALL;
const result: SearchUsers200Response = await searchUsers(tenantId, urlId, usernameStartsWith, mentionGroupIds, sso, searchSection);
[inline-code-end]

---
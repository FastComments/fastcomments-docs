## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| urlId | string | כן |  |
| usernameStartsWith | string | לא |  |
| mentionGroupIds | Array<string> | לא |  |
| sso | string | לא |  |
| searchSection | SearchUsersSearchSectionEnum | לא |  |

## תגובה

מחזיר: [`SearchUsersResult`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SearchUsersResult.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמת searchUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_9f7b3a';
  const urlId: string = 'news/2026/fastcomments-release';
  const usernameStartsWith: string = 'ann';
  const mentionGroupIds: string[] = ['editors', 'contributors'];
  const sso: string = 'google-oauth2';
  const searchSection: SearchUsersSearchSectionEnum = SearchUsersSearchSectionEnum.Mentions;
  const result: SearchUsersResult = await searchUsers(tenantId, urlId, usernameStartsWith, mentionGroupIds, sso, searchSection);
  console.log(result);
})();
[inline-code-end]

---
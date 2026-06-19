## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| usernameStartsWith | string | No |  |
| mentionGroupIds | Array<string> | No |  |
| sso | string | No |  |
| searchSection | SearchUsersSearchSectionEnum | No |  |

## 응답

반환: [`SearchUsersResult`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SearchUsersResult.ts)

## 예제

[inline-code-attrs-start title = 'searchUsers 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
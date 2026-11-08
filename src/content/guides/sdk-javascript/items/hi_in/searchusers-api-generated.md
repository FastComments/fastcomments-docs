## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| urlId | string | हाँ |  |
| usernameStartsWith | string | नहीं |  |
| mentionGroupIds | Array<string> | नहीं |  |
| sso | string | नहीं |  |
| searchSection | SearchUsersSearchSectionEnum | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`SearchUsersResult`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SearchUsersResult.ts)

## उदाहरण

[inline-code-attrs-start title = 'searchUsers उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function example() {
  const tenantId: string = "tenant_12345";
  const urlId: string = "article-9876";
  const usernameStartsWith: string = "john";
  const mentionGroupIds: string[] = ["group1", "group2"];
  const sso: string = "sso_token_abc";
  const searchSection: SearchUsersSearchSectionEnum = SearchUsersSearchSectionEnum.MENTIONS;

  const result: SearchUsersResult = await searchUsers(
    tenantId,
    urlId,
    usernameStartsWith,
    mentionGroupIds,
    sso,
    searchSection
  );

  console.log(result);
}
[inline-code-end]
## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |
| usernameStartsWith | string | 否 |  |
| mentionGroupIds | Array<string> | 否 |  |
| sso | string | 否 |  |
| searchSection | SearchUsersSearchSectionEnum | 否 |  |

## 响应

返回: [`SearchUsersResult`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SearchUsersResult.ts)

## 示例

[inline-code-attrs-start title = 'searchUsers 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
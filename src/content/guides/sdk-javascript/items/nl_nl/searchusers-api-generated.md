## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| usernameStartsWith | string | Nee |  |
| mentionGroupIds | Array<string> | Nee |  |
| sso | string | Nee |  |
| searchSection | SearchUsersSearchSectionEnum | Nee |  |

## Respons

Retourneert: [`SearchUsersResult`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SearchUsersResult.ts)

## Voorbeeld

[inline-code-attrs-start title = 'searchUsers Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
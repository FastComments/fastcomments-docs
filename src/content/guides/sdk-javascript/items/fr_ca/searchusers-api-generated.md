## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| urlId | string | Oui |  |
| usernameStartsWith | string | Non |  |
| mentionGroupIds | Array<string> | Non |  |
| sso | string | Non |  |
| searchSection | SearchUsersSearchSectionEnum | Non |  |

## Réponse

Retourne : [`SearchUsersResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SearchUsersResponse1.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple searchUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoSearch(): Promise<void> {
    const tenantId: string = "tenant_12345";
    const urlId: string = "article-9876";
    const usernameStartsWith: string = "john";
    const mentionGroupIds: string[] = ["groupA", "groupB"];
    const sso: string = "sso_abc123";
    const searchSection: SearchUsersSearchSectionEnum = SearchUsersSearchSectionEnum.Users;

    const response: SearchUsersResponse1 = await searchUsers(
        tenantId,
        urlId,
        usernameStartsWith,
        mentionGroupIds,
        sso,
        searchSection
    );

    console.log(response);
}

demoSearch();
[inline-code-end]

---
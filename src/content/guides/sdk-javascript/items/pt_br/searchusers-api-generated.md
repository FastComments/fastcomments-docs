## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| urlId | string | Sim |  |
| usernameStartsWith | string | Não |  |
| mentionGroupIds | Array<string> | Não |  |
| sso | string | Não |  |
| searchSection | SearchUsersSearchSectionEnum | Não |  |

## Resposta

Retorna: [`SearchUsersResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SearchUsersResponse1.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de searchUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
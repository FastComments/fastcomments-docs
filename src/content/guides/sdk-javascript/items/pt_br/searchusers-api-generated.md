## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-------------|
| tenantId | string | Sim |  |
| urlId | string | Sim |  |
| usernameStartsWith | string | Não |  |
| mentionGroupIds | Array<string> | Não |  |
| sso | string | Não |  |
| searchSection | SearchUsersSearchSectionEnum | Não |  |

## Resposta

Retorna: [`SearchUsers200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SearchUsers200Response.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de searchUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
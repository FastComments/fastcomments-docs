## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-----------|
| tenantId | string | Sim |  |
| urlId | string | Sim |  |
| usernameStartsWith | string | Não |  |
| mentionGroupIds | Array<string> | Não |  |
| sso | string | Não |  |
| searchSection | SearchUsersSearchSectionEnum | Não |  |

## Resposta

Retorna: [`SearchUsersResult`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SearchUsersResult.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de searchUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
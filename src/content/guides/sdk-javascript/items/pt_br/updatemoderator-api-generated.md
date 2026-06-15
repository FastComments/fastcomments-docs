## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| id | string | Sim |  |
| updateModeratorBody | UpdateModeratorBody | Sim |  |

## Resposta

Retorna: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de updateModerator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-81';
const id: string = 'mod_7f3a2b';
const updateModeratorBody: UpdateModeratorBody = {
  email: 'j.reyes@acme-corp.com',
  displayName: 'Jordan Reyes',
  roles: ['moderator', 'content_reviewer'],
  active: true,
  notes: 'Promoted to senior moderator; monitor flagged content weekly'
};
const result: FlagCommentPublic200Response = await updateModerator(tenantId, id, updateModeratorBody);
[inline-code-end]

---
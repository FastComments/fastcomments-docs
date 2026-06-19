---
Informações em massa de usuários para um tenant. Dados os userIds, retorna informações de exibição de User / SSOUser.
Usado pelo widget de comentários para enriquecer usuários que acabaram de aparecer via um evento de presença.
Sem contexto de página: a privacidade é aplicada de forma uniforme (perfis privados são mascarados).

## Parâmetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| ids | string | Sim |  |

## Resposta

Retorna: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersInfoResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getUsersInfo'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_78f9';
const ids: string = 'user_10234,user_10235,user_10236';
const usersInfo: PageUsersInfoResponse = await getUsersInfo(tenantId, ids);
// getUsersInfo requer apenas tenantId e ids; parâmetros opcionais não se aplicam aqui.
[inline-code-end]

---
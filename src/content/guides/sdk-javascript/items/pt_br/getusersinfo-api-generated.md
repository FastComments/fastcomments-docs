Informações de usuário em massa para um tenant. Dado os userIds, retorna informações de exibição do User / SSOUser. Usado pelo widget de comentários para enriquecer usuários que acabaram de aparecer via um evento de presença. Sem contexto de página: a privacidade é aplicada uniformemente (perfís privados são mascarados).

## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| ids | string | Sim |  |

## Resposta

Retorna: [`GetUsersInfoResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUsersInfoResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getUsersInfo'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-tenant";
const ids: string = "user-1001,user-1002";

const usersInfo: GetUsersInfoResponse = await getUsersInfo(tenantId, ids);

// Optional fields in the response may be undefined
const firstUser: PageUserEntry | undefined = usersInfo?.users?.[0];
[inline-code-end]
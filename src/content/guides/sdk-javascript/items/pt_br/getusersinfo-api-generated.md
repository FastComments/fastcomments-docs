Informações de usuário em massa para um locatário. Dado userIds, retorna informações de exibição do User / SSOUser.  
Usado pelo widget de comentários para enriquecer usuários que acabaram de aparecer via um evento de presença.  
Sem contexto de página: a privacidade é aplicada uniformemente (perfís privados são mascarados).

## Parameters

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Yes |  |
| ids | string | Yes |  |

## Response

Returns: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersInfoResponse.ts)

## Example

[inline-code-attrs-start title = 'Exemplo getUsersInfo'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchUsersInfo(): Promise<void> {
  const tenantId: string = "tenant_12345";
  const ids: string = "user_001,user_002";
  const response: PageUsersInfoResponse = await getUsersInfo(tenantId, ids);
  console.log(response);
}
fetchUsersInfo();
[inline-code-end]
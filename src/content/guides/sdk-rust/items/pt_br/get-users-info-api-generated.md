Informações em lote de usuários para um tenant. Dados os userIds, retorna informações de exibição do User / SSOUser.  
Usado pelo widget de comentários para enriquecer usuários que acabaram de aparecer via um evento de presença.  
Sem contexto de página: a privacidade é aplicada uniformemente (perfís privados são mascarados).

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| ids | String | Yes |  |

## Response

Retorna: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_info_response.rs)

## Example

[inline-code-attrs-start title = 'Exemplo get_users_info'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params = GetUsersInfoParams {
    tenant_id: "acme-corp-tenant".to_string(),
    ids: "user-1,user-2".to_string(),
};
let page: PageUsersInfoResponse = get_users_info(&configuration, params).await?;
[inline-code-end]
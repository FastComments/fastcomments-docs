## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-------------|
| tenant_id | String | Sim |  |
| id | String | Sim |  |
| sure | String | Não |  |

## Resposta

Retorna: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de delete_tenant'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: DeleteTenantParams = DeleteTenantParams {
    tenant_id: String::from("acme-corp-tenant"),
    id: String::from("acme-corp-tenant-001"),
    sure: Some(String::from("confirm-delete")),
};
let response: FlagCommentPublic200Response = delete_tenant(&configuration, params).await?;
[inline-code-end]

---
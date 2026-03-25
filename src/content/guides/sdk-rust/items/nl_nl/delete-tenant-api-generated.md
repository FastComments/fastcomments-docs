## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| id | String | Ja |  |
| sure | String | Nee |  |

## Respons

Retourneert: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Voorbeeld

[inline-code-attrs-start title = 'delete_tenant Voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: DeleteTenantParams = DeleteTenantParams {
    tenant_id: String::from("acme-corp-tenant"),
    id: String::from("acme-corp-tenant-001"),
    sure: Some(String::from("confirm-delete")),
};
let response: FlagCommentPublic200Response = delete_tenant(&configuration, params).await?;
[inline-code-end]

---
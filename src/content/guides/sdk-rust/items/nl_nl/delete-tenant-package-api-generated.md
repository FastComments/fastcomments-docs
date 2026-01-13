## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|---------|-------------|
| tenant_id | String | Ja |  |
| id | String | Ja |  |

## Respons

Geeft terug: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Voorbeeld

[inline-code-attrs-start title = 'delete_tenant_package Voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: DeleteTenantPackageParams = DeleteTenantPackageParams {
    tenant_id: "acme-corp-tenant".to_string(),
    id: "pkg-news-comments-2025-01".to_string(),
    cascade: Some(true),
};
let response: FlagCommentPublic200Response = delete_tenant_package(&configuration, params).await?;
[inline-code-end]
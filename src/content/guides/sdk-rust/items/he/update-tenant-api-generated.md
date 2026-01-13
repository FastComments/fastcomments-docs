## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| id | String | כן |  |
| update_tenant_body | models::UpdateTenantBody | כן |  |

## תגובה

מחזיר: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-update_tenant'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_update_tenant() -> Result<FlagCommentPublic200Response, Error> {
    let params: UpdateTenantParams = UpdateTenantParams {
        tenant_id: "acme-corp-tenant".to_owned(),
        id: "site-42".to_owned(),
        update_tenant_body: models::UpdateTenantBody {
            name: Some("Acme Corporation".to_string()),
            default_site: Some("news/article".to_string()),
            allowed_origins: Some(vec![
                "https://www.acme.com".to_string(),
                "https://blog.acme.com".to_string(),
            ]),
            invite_only: Some(false),
            api_domain_configuration: Some(ApiDomainConfiguration {
                domain: "comments.acme.com".to_string(),
                secure: Some(true),
                ..Default::default()
            }),
            ..Default::default()
        },
    };
    let response: FlagCommentPublic200Response = update_tenant(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---
## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| domain_to_update | String | Yes |  |
| update_domain_config_params | models::UpdateDomainConfigParams | Yes |  |

## 回應

Returns: [`PutDomainConfigResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/put_domain_config_response.rs)

## 範例

[inline-code-attrs-start title = 'put_domain_config 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn update_domain(configuration: &configuration::Configuration) -> Result<(), Error> {
    let update_params = models::UpdateDomainConfigParams {
        enable_comments: Some(true),
        moderation_level: Some("strict".to_string()),
        max_comment_length: Some(500),
        ..Default::default()
    };
    let params = PutDomainConfigParams {
        tenant_id: "acme-corp-tenant".to_string(),
        domain_to_update: "news.example.com".to_string(),
        update_domain_config_params: update_params,
    };
    let _resp: PutDomainConfigResponse = put_domain_config(configuration, params).await?;
    Ok(())
}
[inline-code-end]

---
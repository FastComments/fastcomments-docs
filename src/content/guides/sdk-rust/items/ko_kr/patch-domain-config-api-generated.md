## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| domain_to_update | String | 예 |  |
| patch_domain_config_params | models::PatchDomainConfigParams | 예 |  |

## 응답

반환: [`PatchDomainConfigResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/patch_domain_config_response.rs)

## 예제

[inline-code-attrs-start title = 'patch_domain_config 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: PatchDomainConfigParams = PatchDomainConfigParams {
        tenant_id: "acme-corp-tenant".to_string(),
        domain_to_update: "news/article".to_string(),
        patch_domain_config_params: models::PatchDomainConfigParams {
            allowed_origins: Some(vec![
                "https://www.acme.com".to_string(),
                "https://blog.acme.com".to_string(),
            ]),
            enable_moderation: Some(true),
            moderation_mode: Some("pre".to_string()),
            webhook_url: Some("https://hooks.acme.com/comments".to_string()),
            max_comment_length: Some(1000),
        },
    };
    let response: PatchDomainConfigResponse = patch_domain_config(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---
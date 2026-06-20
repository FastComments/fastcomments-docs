## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| add_domain_config_params | models::AddDomainConfigParams | 예 |  |

## 응답

반환: [`AddDomainConfigResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/add_domain_config_response.rs)

## 예제

[inline-code-attrs-start title = 'add_domain_config 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: AddDomainConfigParams = AddDomainConfigParams {
        tenant_id: "acme-corp-tenant".to_string(),
        add_domain_config_params: models::AddDomainConfigParams {
            domain: "news.example.com".to_string(),
            path_prefix: Some("news/article".to_string()),
            allow_subdomains: Some(true),
            allowed_origins: Some(vec![
                "https://www.example.com".to_string(),
                "https://editor.example.com".to_string()
            ]),
            default_moderation: Some("pre-moderation".to_string()),
            enabled: Some(true),
        },
    };

    let response: AddDomainConfigResponse = add_domain_config(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---
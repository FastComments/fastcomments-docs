## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| domain | String | Yes |  |

## 응답

반환: [`GetDomainConfigResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_domain_config_response.rs)

## 예시

[inline-code-attrs-start title = 'get_domain_config 예시'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
#[tokio::main]
async fn main() -> Result<(), Error> {
    let params = GetDomainConfigParams {
        tenant_id: "acme-corp-tenant".to_string(),
        domain: "news/article".to_string(),
        locale: Some("en-US".to_string()),
    };
    let _response = get_domain_config(&config, params).await?;
    Ok(())
}
[inline-code-end]

---
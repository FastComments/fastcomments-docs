## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|------|------|
| tenant_id | String | 是 |  |
| domain | String | 是 |  |

## 回應

返回: [`GetDomainConfigResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_domain_config_response.rs)

## 範例

[inline-code-attrs-start title = 'get_domain_config 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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
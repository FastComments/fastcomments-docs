## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| url_id | String | Yes |  |
| title | String | No |  |

## 回應

返回: `CreateV1PageReact`

## 範例

[inline-code-attrs-start title = 'create_v1_page_react 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: CreateV1PageReactParams = CreateV1PageReactParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article".to_string(),
        title: Some("Rust Community Update".to_string()),
    };
    let _response = create_v1_page_react(&config, params).await?;
    Ok(())
}
[inline-code-end]
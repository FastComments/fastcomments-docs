## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| url_id | String | 是 |  |
| title | String | 否 |  |

## 回應

回傳: `CreateV1PageReact`

## 範例

[inline-code-attrs-start title = 'create_v1_page_react 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<CreateV1PageReact, Error> {
    let params = CreateV1PageReactParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article/2026/06/launch".to_string(),
        title: Some("Acme Launch Coverage".to_string()),
    };
    let reaction: CreateV1PageReact = create_v1_page_react(&configuration, params).await?;
    Ok(reaction)
}
[inline-code-end]

---
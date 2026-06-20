## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| url_id | String | 是 |  |
| id | String | 是 |  |

## 响应

返回: `CreateV1PageReact`

## 示例

[inline-code-attrs-start title = 'delete_v2_page_react 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_delete() -> Result<(), Error> {
    let params: DeleteV2PageReactParams = DeleteV2PageReactParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article/2026/06/feature-ml".to_string(),
        id: "react_987654321".to_string(),
    };
    let request_id: Option<String> = Some("req-20260619-01".to_string());
    let deleted: CreateV1PageReact = delete_v2_page_react(&configuration, params).await?;
    let _ = request_id;
    Ok(())
}
[inline-code-end]

---
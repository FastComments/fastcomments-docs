列出租户的页面。由 FChat 桌面客户端使用以填充其房间列表。
要求每个页面的已解析自定义配置中 `enableFChat` 为 true。
需要 SSO 的页面会根据请求用户的组访问权限进行过滤。

## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| cursor | String | 否 |  |
| limit | i32 | 否 |  |
| q | String | 否 |  |
| sort_by | models::PagesSortBy | 否 |  |
| has_comments | bool | 否 |  |

## 响应

返回:[`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_public_pages_response.rs)

## 示例

[inline-code-attrs-start title = 'get_pages_public 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = GetPagesPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        cursor: Some("page_20".to_string()),
        limit: Some(50),
        q: Some("news/article".to_string()),
        sort_by: Some(models::PagesSortBy::CreatedDesc),
        has_comments: Some(true),
    };
    let _response = get_pages_public(configuration, params).await?;
    Ok(())
}
[inline-code-end]
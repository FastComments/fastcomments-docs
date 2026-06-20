## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| url_id | String | 是 |  |

## 响应

返回：[`GetPageByUrlidApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_page_by_urlid_api_response.rs)

## 示例

[inline-code-attrs-start title = 'get_page_by_urlid 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_page() -> Result<GetPageByUrlidApiResponse, Error> {
    let params: GetPageByUrlidParams = GetPageByUrlidParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article/how-to-build-an-api".to_string(),
        locale: Some("en-US".to_string()),
    };
    let page: GetPageByUrlidApiResponse = get_page_by_urlid(&configuration, params).await?;
    Ok(page)
}
[inline-code-end]

---
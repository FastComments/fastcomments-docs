## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| url_id | String | 예 |  |

## 응답

반환: [`GetPageByUrlidApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_page_by_urlid_api_response.rs)

## 예제

[inline-code-attrs-start title = 'get_page_by_urlid 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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
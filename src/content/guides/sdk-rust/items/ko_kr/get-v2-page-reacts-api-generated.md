## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| url_id | String | 예 |  |

## 응답

반환: `GetV2PageReacts`

## 예시

[inline-code-attrs-start title = 'get_v2_page_reacts 예시'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params = GetV2PageReactsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article".to_string(),
        page: Some(1),
        page_size: Some(50),
    };
    let _reacts = get_v2_page_reacts(&configuration, params).await?;
    Ok(())
}
[inline-code-end]
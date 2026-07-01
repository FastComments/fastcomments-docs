## 參數

| 名稱 | 類型 | 必要 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| url_id | String | Yes |  |

## 回應

返回： `GetV2PageReacts`

## 範例

[inline-code-attrs-start title = 'get_v2_page_reacts 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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

---
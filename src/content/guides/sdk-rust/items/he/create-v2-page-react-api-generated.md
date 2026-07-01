## פרמטרים

| שם | סוג | דרוש | תיאור |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| url_id | String | Yes |  |
| id | String | Yes |  |
| title | String | No |  |

## תגובה

מחזיר: `CreateV1PageReact`

## דוגמה

[inline-code-attrs-start title = 'דוגמת create_v2_page_react'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params = CreateV2PageReactParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article".to_string(),
        id: "comment-123".to_string(),
        title: Some("Breaking News".to_string()),
    };
    let _react = create_v2_page_react(&configuration, params).await?;
    Ok(())
}
[inline-code-end]
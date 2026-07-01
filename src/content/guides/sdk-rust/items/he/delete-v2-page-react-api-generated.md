## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| url_id | String | Yes |  |
| id | String | Yes |  |

## תגובה

מחזיר: `CreateV1PageReact`

## דוגמה

[inline-code-attrs-start title = 'דוגמה של delete_v2_page_react'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params = DeleteV2PageReactParams {
        tenant_id: "acme-corp-tenant".into(),
        url_id: "news/article".into(),
        id: "react-987".into(),
    };
    let _response: CreateV1PageReact = delete_v2_page_react(&config, params).await?;
    Ok(())
}
[inline-code-end]
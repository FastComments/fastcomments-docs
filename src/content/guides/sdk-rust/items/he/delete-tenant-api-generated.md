## פרמטרים

| שם | סוג | חובה | תיאור |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| id | String | כן |  |
| sure | String | לא |  |

## תגובה

מחזירה: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## דוגמה

[inline-code-attrs-start title = 'דוגמת delete_tenant'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn delete_example(config: &configuration::Configuration) -> Result<(), Error> {
    let params = DeleteTenantParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/article".to_string(),
        sure: Some("true".to_string()),
    };
    delete_tenant(config, params).await?;
    Ok(())
}
[inline-code-end]
רשימת עמודים עבור טננט. משמש את לקוח שולחן העבודה של FChat לאכלוס רשימת החדרים שלו.
דורש ש-`enableFChat` יהיה true בקונפיגורציה המותאמת שנפתרה עבור כל עמוד.
עמודים הדורשים SSO מסוננים מול הרשאות הקבוצה של המשתמש המבקש.

## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| cursor | String | לא |  |
| limit | i32 | לא |  |
| q | String | לא |  |
| sort_by | models::PagesSortBy | לא |  |
| has_comments | bool | לא |  |

## תגובה

מחזיר: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_public_pages_response.rs)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_pages_public'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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
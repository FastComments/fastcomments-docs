רשימת דפים לשוכר. משמשת את לקוח השולחן עבודה של FChat למילוי רשימת החדרים שלו.
נדרש ש-`enableFChat` יהיה true בתצורת ה-custom שנפתרה עבור כל דף.
דפים שדורשים SSO מסוננים לפי גישת הקבוצה של המשתמש המבקש.

## פרמטרים

| Name | Type | Required | Description |
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
let params: GetPagesPublicParams = GetPagesPublicParams {
    tenant_id: String::from("acme-corp-tenant"),
    cursor: Some(String::from("cursor_eyJwZl9pZCI6IjEyMyJ9")),
    limit: Some(50),
    q: Some(String::from("tag:release status:published")),
    sort_by: Some(models::PagesSortBy::CreatedAt),
    has_comments: Some(true),
};
let response: GetPublicPagesResponse = get_pages_public(&configuration, params).await?;
[inline-code-end]

---
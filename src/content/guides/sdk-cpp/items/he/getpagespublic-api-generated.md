---
רשימת דפים עבור tenant. משמשת את לקוח שולחן העבודה של FChat כדי למלא את רשימת החדרים שלו.
דורש ש־`enableFChat` יהיה true בקונפיגורציית ה-custom שנקבעה עבור כל דף.
דפים שדורשים SSO מסוננים בהתאם לגישת הקבוצה של המשתמש המבקש.

## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| cursor | string | לא |  |
| limit | int32_t | לא |  |
| q | string | לא |  |
| sortBy | PagesSortBy | לא |  |
| hasComments | bool | לא |  |

## תגובה

מחזיר: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetPublicPagesResponse.h)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getPagesPublic'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
boost::optional<utility::string_t> cursor = utility::string_t(U("cursor_abc"));
boost::optional<int32_t> limit = 50;
boost::optional<utility::string_t> q = utility::string_t(U("status:published"));
boost::optional<PagesSortBy> sortBy = PagesSortBy::NEWEST;
boost::optional<bool> hasComments = true;
api->getPagesPublic(tenantId, cursor, limit, q, sortBy, hasComments)
.then([](std::shared_ptr<GetPublicPagesResponse> resp){
    if (!resp) resp = std::make_shared<GetPublicPagesResponse>();
})
.wait();
[inline-code-end]

---
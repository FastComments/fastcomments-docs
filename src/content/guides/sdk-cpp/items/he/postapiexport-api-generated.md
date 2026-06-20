## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| textSearch | string | לא |  |
| byIPFromComment | string | לא |  |
| filters | string | לא |  |
| searchFilters | string | לא |  |
| sorts | string | לא |  |
| sso | string | לא |  |

## תשובה

מחזיר: [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationExportResponse.h)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-postApiExport'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> textSearch(utility::string_t("spam content"));
boost::optional<utility::string_t> byIPFromComment(utility::string_t("203.0.113.45"));
boost::optional<utility::string_t> filters(utility::string_t("site:my-tenant-123 status:pending"));
boost::optional<utility::string_t> searchFilters(utility::string_t("created>2026-01-01"));
boost::optional<utility::string_t> sorts(utility::string_t("created:desc"));
boost::optional<utility::string_t> sso(utility::string_t("user@example.com"));

auto task = api->postApiExport(textSearch, byIPFromComment, filters, searchFilters, sorts, sso)
.then([](pplx::task<std::shared_ptr<ModerationExportResponse>> t){
    try {
        auto resp = t.get();
        return resp ? std::make_shared<ModerationExportResponse>(*resp) : std::make_shared<ModerationExportResponse>();
    } catch(...) {
        return std::shared_ptr<ModerationExportResponse>();
    }
});

task.wait();
[inline-code-end]
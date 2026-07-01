## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| id | string | כן |  |
| updateAPIPageData | UpdateAPIPageData | כן |  |

## תגובה

מחזיר: [`PatchPageAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PatchPageAPIResponse.h)

## דוגמה

[inline-code-attrs-start title = 'patchPage דוגמה'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t pageId = U("page-789");
UpdateAPIPageData update;
update.title = boost::optional<utility::string_t>(U("Updated Page Title"));
update.metadata = boost::optional<utility::string_t>(U("{\"author\":\"jane.doe@example.com\"}"));
api->patchPage(tenantId, pageId, update)
    .then([](std::shared_ptr<PatchPageAPIResponse> response) {
        if (response && response->isSuccess) {
            // טיפול בהצלחה
        }
    })
    .then([](pplx::task<void> t) {
        try { t.get(); } catch (const std::exception&) {}
    });
[inline-code-end]
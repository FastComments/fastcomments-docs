## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| id | string | כן |  |

## תגובה

מחזירה: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## דוגמה

[inline-code-attrs-start title = 'דוגמת deletePendingWebhookEvent'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto eventId = utility::conversions::to_string_t("event-987654");
boost::optional<utility::string_t> optTenant = tenantId;

api->deletePendingWebhookEvent(optTenant.value(), eventId)
    .then([](std::shared_ptr<APIEmptyResponse> resp) {
        (void)resp;
    })
    .then([](pplx::task<void> t) {
        try { t.get(); } catch (const std::exception& e) { }
    });
[inline-code-end]
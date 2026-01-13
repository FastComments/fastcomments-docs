## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| id | string | כן |  |

## תגובה

מחזיר: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-deletePendingWebhookEvent'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t eventId = U("wh_evt_456");
boost::optional<utility::string_t> requestId = boost::optional<utility::string_t>(U("req-789"));
api->deletePendingWebhookEvent(tenantId, eventId)
    .then([requestId](pplx::task<std::shared_ptr<FlagCommentPublic_200_response>> t) {
        try {
            auto resp = t.get();
            if (resp) {
                auto resultCopy = std::make_shared<FlagCommentPublic_200_response>(*resp);
                if (requestId) {
                    (void)requestId.get();
                }
                (void)resultCopy;
            } else {
                auto fallback = std::make_shared<FlagCommentPublic_200_response>();
                (void)fallback;
            }
        } catch (const std::exception&) {
            auto errorObj = std::make_shared<FlagCommentPublic_200_response>();
            (void)errorObj;
        }
    }).wait();
[inline-code-end]

---
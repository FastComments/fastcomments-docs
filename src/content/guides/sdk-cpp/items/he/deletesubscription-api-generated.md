## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| id | string | כן |  |
| userId | string | לא |  |

## תגובה

מחזיר: [`DeleteSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/DeleteSubscriptionAPIResponse.h)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-deleteSubscription'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t id = U("subscription-987");
boost::optional<utility::string_t> userId = boost::optional<utility::string_t>(U("user@example.com"));
auto defaultResp = std::make_shared<DeleteSubscriptionAPIResponse>();
api->deleteSubscription(tenantId, id, userId)
.then([defaultResp](pplx::task<std::shared_ptr<DeleteSubscriptionAPIResponse>> t){
    try {
        auto resp = t.get();
        if (!resp) resp = defaultResp;
        std::cout << "Delete completed\n";
    } catch (const std::exception &e) {
        std::cerr << "Delete failed: " << e.what() << '\n';
    }
});
[inline-code-end]

---
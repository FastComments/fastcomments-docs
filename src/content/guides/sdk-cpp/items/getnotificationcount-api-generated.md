## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| userId | string | No |  |
| urlId | string | No |  |
| fromCommentId | string | No |  |
| viewed | bool | No |  |
| type | string | No |  |

## Response

Returns: [`GetNotificationCount_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetNotificationCount_200_response.h)

## Example

[inline-code-attrs-start title = 'getNotificationCount Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId(U("my-tenant-123"));
boost::optional<utility::string_t> userId = utility::string_t(U("user@example.com"));
boost::optional<utility::string_t> urlId = utility::string_t(U("https://example.com/articles/456"));
boost::optional<utility::string_t> fromCommentId = utility::string_t(U("cmt-789"));
boost::optional<bool> viewed = true;
boost::optional<utility::string_t> type = utility::string_t(U("reply"));
api->getNotificationCount(tenantId, userId, urlId, fromCommentId, viewed, type)
    .then([](pplx::task<std::shared_ptr<GetNotificationCount_200_response>> t){
        try {
            auto resp = t.get();
            if (resp) std::cout << "Notification count response received\n";
            else std::cout << "No response\n";
        } catch (const std::exception& e) {
            std::cerr << "Request failed: " << e.what() << '\n';
        }
    });
[inline-code-end]

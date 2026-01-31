## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateNotificationBody | UpdateNotificationBody | Yes |  |
| userId | string | No |  |

## Response

Returns: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## Example

[inline-code-attrs-start title = 'updateNotification Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
auto id = utility::string_t(U("notif-456"));
auto body = std::make_shared<UpdateNotificationBody>();
boost::optional<utility::string_t> userId = boost::optional<utility::string_t>(utility::string_t(U("user@example.com")));
api->updateNotification(tenantId, id, *body, userId)
    .then([](pplx::task<std::shared_ptr<FlagCommentPublic_200_response>> t){
        try {
            auto resp = t.get();
            if (resp) {
                std::cout << "Notification updated successfully\n";
            }
        } catch (const std::exception &e) {
            std::cerr << "Update failed: " << e.what() << '\n';
        }
    });
[inline-code-end]

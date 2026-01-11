## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateUserBadgeParams | UpdateUserBadgeParams | Yes |  |

## Response

Returns: [`UpdateUserBadge_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UpdateUserBadge_200_response.h)

## Example

[inline-code-attrs-start title = 'updateUserBadge Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t id = U("user-9876");
auto paramsPtr = std::make_shared<UpdateUserBadgeParams>();
paramsPtr->name = utility::string_t(U("Community Moderator"));
paramsPtr->color = utility::string_t(U("#4CAF50"));
boost::optional<utility::string_t> note = boost::optional<utility::string_t>(U("Recognized contributor"));
paramsPtr->note = note;
paramsPtr->active = true;
api->updateUserBadge(tenantId, id, *paramsPtr)
    .then([](pplx::task<std::shared_ptr<UpdateUserBadge_200_response>> task) {
        try {
            auto resp = task.get();
            (void)resp;
        } catch (...) {
        }
    });
[inline-code-end]

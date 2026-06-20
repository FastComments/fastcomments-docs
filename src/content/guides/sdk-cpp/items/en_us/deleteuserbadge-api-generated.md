## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`APIEmptySuccessResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptySuccessResponse.h)

## Example

[inline-code-attrs-start title = 'deleteUserBadge Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t badgeId = U("badge-456");
boost::optional<utility::string_t> requestId = boost::optional<utility::string_t>(U("req-789"));
api->deleteUserBadge(tenantId, badgeId)
    .then([tenantId, requestId](std::shared_ptr<APIEmptySuccessResponse> resp) {
        auto result = resp ? resp : std::make_shared<APIEmptySuccessResponse>();
        std::cout << "Deleted badge '" << badgeId << "' for tenant '" << tenantId << "'\n";
        return result;
    })
    .then([](std::shared_ptr<APIEmptySuccessResponse>){
    })
    .wait();
[inline-code-end]

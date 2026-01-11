## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| userId | string | No |  |
| limit | double | No |  |
| skip | double | No |  |

## Response

Returns: [`GetUserBadgeProgressList_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetUserBadgeProgressList_200_response.h)

## Example

[inline-code-attrs-start title = 'getUserBadgeProgressList Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> userId(U("user@example.com"));
boost::optional<double> limit(25);
boost::optional<double> skip(0);
api->getUserBadgeProgressList(tenantId, userId, limit, skip)
.then([](pplx::task<std::shared_ptr<GetUserBadgeProgressList_200_response>> t)
{
    try {
        auto resp = t.get();
        return resp ? resp : std::make_shared<GetUserBadgeProgressList_200_response>();
    } catch(...) {
        return std::make_shared<GetUserBadgeProgressList_200_response>();
    }
});
[inline-code-end]

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| userId | string | No |  |
| badgeId | string | No |  |
| type | double | No |  |
| displayedOnComments | bool | No |  |
| limit | double | No |  |
| skip | double | No |  |

## Response

Returns: [`GetUserBadges_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetUserBadges_200_response.h)

## Example

[inline-code-attrs-start title = 'getUserBadges Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> userId = boost::optional<utility::string_t>(U("user@example.com"));
boost::optional<utility::string_t> badgeId = boost::optional<utility::string_t>(U("badge-987"));
boost::optional<double> type = boost::optional<double>(1.0);
boost::optional<bool> displayedOnComments = boost::optional<bool>(true);
boost::optional<double> limit = boost::optional<double>(50.0);
boost::optional<double> skip = boost::optional<double>(0.0);

api->getUserBadges(tenantId, userId, badgeId, type, displayedOnComments, limit, skip)
.then([](pplx::task<std::shared_ptr<GetUserBadges_200_response>> task){
    try {
        auto resp = task.get();
        if (resp) {
            auto localCopy = std::make_shared<GetUserBadges_200_response>(*resp);
            (void)localCopy;
        }
    } catch (const std::exception&) {}
});
[inline-code-end]

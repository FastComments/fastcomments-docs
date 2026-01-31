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
utility::string_t userId = U("user@example.com");
utility::string_t badgeId = U("badge-elite");
boost::optional<utility::string_t> optUser(userId);
boost::optional<utility::string_t> optBadge(badgeId);
boost::optional<double> optType(1.0);
boost::optional<bool> optDisplayed(true);
boost::optional<double> optLimit(25);
boost::optional<double> optSkip(0);
auto holder = std::make_shared<std::shared_ptr<GetUserBadges_200_response>>();
api->getUserBadges(tenantId, optUser, optBadge, optType, optDisplayed, optLimit, optSkip)
.then([holder](pplx::task<std::shared_ptr<GetUserBadges_200_response>> t){
    try { *holder = t.get(); } catch(...) {}
});
[inline-code-end]

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| sso | string | No |  |

## Response

Returns: [`APIModerateGetUserBanPreferencesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIModerateGetUserBanPreferencesResponse.h)

## Example

[inline-code-attrs-start title = 'getUserBanPreference Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));

api->getUserBanPreference(tenantId, sso)
    .then([](pplx::task<std::shared_ptr<APIModerateGetUserBanPreferencesResponse>> t) {
        try {
            auto resp = t.get();
        } catch (const std::exception&) {
        }
    });
[inline-code-end]

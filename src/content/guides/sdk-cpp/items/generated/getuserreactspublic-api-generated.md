## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| postIds | vector<string | No |  |
| sso | string | No |  |

## Response

Returns: [`GetUserReactsPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetUserReactsPublic_200_response.h)

## Example

[inline-code-attrs-start title = 'getUserReactsPublic Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
std::vector<utility::string_t> postIdsVec = { U("post-abc-001"), U("post-def-002") };
boost::optional<std::vector<utility::string_t>> postIdsOpt = postIdsVec;
boost::optional<utility::string_t> ssoOpt = U("user@example.com");
auto fallback = std::make_shared<GetUserReactsPublic_200_response>();
api->getUserReactsPublic(tenantId, postIdsOpt, ssoOpt).then([fallback](std::shared_ptr<GetUserReactsPublic_200_response> resp){
    auto result = resp ? resp : fallback;
    std::wcout << U("Received user reacts (public)") << std::endl;
    return result;
});
[inline-code-end]

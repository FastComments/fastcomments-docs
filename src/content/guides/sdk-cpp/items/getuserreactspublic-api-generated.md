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
boost::optional<std::vector<utility::string_t>> postIds = std::vector<utility::string_t>{ U("post-001"), U("post-002") };
boost::optional<utility::string_t> sso = U("user@example.com");
api->getUserReactsPublic(tenantId, postIds, sso)
.then([](pplx::task<std::shared_ptr<GetUserReactsPublic_200_response>> t){
    auto resp = t.get();
    auto result = resp ? resp : std::make_shared<GetUserReactsPublic_200_response>();
    std::cout << "User reacts received" << std::endl;
});
[inline-code-end]

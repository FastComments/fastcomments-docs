## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createUserBadgeParams | CreateUserBadgeParams | Yes |  |

## Response

Returns: [`CreateUserBadge_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateUserBadge_200_response.h)

## Example

[inline-code-attrs-start title = 'createUserBadge Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = utility::string_t("my-tenant-123");
CreateUserBadgeParams params;
params.userId = utility::string_t("user@example.com");
params.badgeId = utility::string_t("gold-contributor");
boost::optional<utility::string_t> expiresAt = utility::string_t("2026-12-31T23:59:59Z");
params.expiresAt = expiresAt;
auto placeholder = std::make_shared<CreateUserBadge_200_response>();
api->createUserBadge(tenantId, params).then([](pplx::task<std::shared_ptr<CreateUserBadge_200_response>> t){
    try {
        auto resp = t.get();
        (void)resp;
    } catch (const std::exception&) {
    }
});
[inline-code-end]

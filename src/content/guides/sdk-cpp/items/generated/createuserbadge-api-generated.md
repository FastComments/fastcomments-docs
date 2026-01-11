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
utility::string_t tenantId = U("my-tenant-123");
auto params = std::make_shared<CreateUserBadgeParams>();
params->userId = utility::string_t(U("user@example.com"));
params->badgeId = utility::string_t(U("top-contributor"));
params->awardedBy = boost::optional<utility::string_t>(utility::string_t(U("moderator@example.com")));
params->expiresAt = boost::optional<utility::string_t>(utility::string_t(U("2026-12-31T23:59:59Z")));
api->createUserBadge(tenantId, *params)
    .then([](pplx::task<std::shared_ptr<CreateUserBadge_200_response>> t) {
        try {
            auto resp = t.get();
            (void)resp;
        } catch (const std::exception&) {
        }
    });
[inline-code-end]

## Parametre

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| updateUserBadgeParams | UpdateUserBadgeParams | Ja |  |

## Svar

Returnerer: [`APIEmptySuccessResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptySuccessResponse.h)

## Eksempel

[inline-code-attrs-start title = 'updateUserBadge Eksempel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t userId = U("user@example.com");
auto params = std::make_shared<UpdateUserBadgeParams>();
params->badgeId = U("badge-987");
params->label = boost::optional<utility::string_t>(U("Top Contributor"));
params->expiresAt = boost::optional<utility::string_t>(U("2026-12-31T23:59:59Z"));
params->active = boost::optional<bool>(true);
api->updateUserBadge(tenantId, userId, *params)
    .then([](pplx::task<std::shared_ptr<APIEmptySuccessResponse>> t){
        try {
            auto resp = t.get();
            (void)resp;
        } catch (const std::exception&) {
        }
    });
[inline-code-end]

---
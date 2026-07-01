## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Yes |  |
| createUserBadgeParams | CreateUserBadgeParams | Yes |  |

## Antwort

Rückgabe: [`APICreateUserBadgeResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APICreateUserBadgeResponse.h)

## Beispiel

[inline-code-attrs-start title = 'createUserBadge Beispiel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = U("my-tenant-123");
CreateUserBadgeParams badgeParams;
badgeParams.userEmail = U("user@example.com");
badgeParams.badgeCode = U("gold");
badgeParams.expirationDate = boost::optional<utility::datetime>(utility::datetime::utc_now() + utility::datetime::from_seconds(2592000));
api->createUserBadge(tenantId, badgeParams).then([](std::shared_ptr<APICreateUserBadgeResponse> resp){
    if (resp && resp->success) {
        auto result = std::make_shared<APICreateUserBadgeResponse>(*resp);
    }
});
[inline-code-end]
## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlIdWS | string | Ja |  |
| userIds | string | Ja |  |

## Svar

Returnerer: [`GetUserPresenceStatusesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetUserPresenceStatusesResponse.h)

## Eksempel

[inline-code-attrs-start title = 'getUserPresenceStatuses Eksempel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = U("my-tenant-123");
auto urlIdWS = U("article-789");
auto userIds = U("alice@example.com,bob@example.com");
boost::optional<utility::string_t> optionalFilter = boost::none;
api->getUserPresenceStatuses(tenantId, urlIdWS, userIds)
    .then([](pplx::task<std::shared_ptr<GetUserPresenceStatusesResponse>> t){
        try{
            auto response = t.get();
        }catch(...){
        }
    });
[inline-code-end]
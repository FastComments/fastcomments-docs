## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|------------|-----------|
| tenantId | string | Yes |  |
| urlIdWS | string | Yes |  |
| userIds | string | Yes |  |

## Απάντηση

Επιστρέφει: [`GetUserPresenceStatusesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetUserPresenceStatusesResponse.h)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getUserPresenceStatuses'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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
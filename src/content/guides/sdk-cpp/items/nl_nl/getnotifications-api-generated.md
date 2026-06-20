## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| userId | string | Nee |  |
| urlId | string | Nee |  |
| fromCommentId | string | Nee |  |
| viewed | bool | Nee |  |
| type | string | Nee |  |
| skip | double | Nee |  |

## Respons

Geeft terug: [`GetNotificationsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetNotificationsResponse.h)

## Voorbeeld

[inline-code-attrs-start title = 'getNotifications Voorbeeld'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> userId(U("user@example.com"));
boost::optional<utility::string_t> urlId(U("/articles/2026/new-feature"));
boost::optional<utility::string_t> fromCommentId(U("cmt-98765"));
boost::optional<bool> viewed(true);
boost::optional<utility::string_t> type(U("reply"));
boost::optional<double> skip(0.0);

api->getNotifications(tenantId, userId, urlId, fromCommentId, viewed, type, skip)
.then([](std::shared_ptr<GetNotificationsResponse> resp){
    auto holder = std::make_shared<GetNotificationsResponse>();
    holder = resp;
    if (holder) std::cout << "Received notifications\n";
    return holder;
});
[inline-code-end]

---
## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| userId | string | No |  |
| urlId | string | No |  |
| fromCommentId | string | No |  |
| viewed | bool | No |  |
| type | string | No |  |
| skip | double | No |  |

## Risposta

Restituisce: [`GetNotificationsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetNotificationsResponse.h)

## Esempio

[inline-code-attrs-start title = 'Esempio di getNotifications'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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
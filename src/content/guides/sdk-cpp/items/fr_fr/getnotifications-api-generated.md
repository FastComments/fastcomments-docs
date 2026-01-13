## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| userId | string | Non |  |
| urlId | string | Non |  |
| fromCommentId | string | Non |  |
| viewed | bool | Non |  |
| type | string | Non |  |
| skip | double | Non |  |

## Réponse

Renvoie : [`GetNotifications_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetNotifications_200_response.h)

## Exemple

[inline-code-attrs-start title = 'Exemple getNotifications'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> userId = boost::optional<utility::string_t>(U("user@example.com"));
boost::optional<utility::string_t> urlId = boost::optional<utility::string_t>(U("https://example.com/posts/42"));
boost::optional<utility::string_t> fromCommentId = boost::optional<utility::string_t>(U("cmt-98765"));
boost::optional<bool> viewed = boost::optional<bool>(true);
boost::optional<utility::string_t> type = boost::optional<utility::string_t>(U("reply"));
boost::optional<double> skip = boost::optional<double>(0.0);

api->getNotifications(tenantId, userId, urlId, fromCommentId, viewed, type, skip)
.then([](pplx::task<std::shared_ptr<GetNotifications_200_response>> t){
    try {
        auto resp = t.get();
        if (resp) {
            auto processed = std::make_shared<GetNotifications_200_response>(*resp);
        }
    } catch (const std::exception&) {
    }
});
[inline-code-end]

---
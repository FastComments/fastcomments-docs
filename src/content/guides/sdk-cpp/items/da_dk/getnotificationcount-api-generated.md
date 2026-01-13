## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| userId | string | Nej |  |
| urlId | string | Nej |  |
| fromCommentId | string | Nej |  |
| viewed | bool | Nej |  |
| type | string | Nej |  |

## Respons

Returnerer: [`GetNotificationCount_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetNotificationCount_200_response.h)

## Eksempel

[inline-code-attrs-start title = 'getNotificationCount Eksempel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> userId = U("user@example.com");
boost::optional<utility::string_t> urlId = U("https://www.example.com/article/456");
boost::optional<utility::string_t> fromCommentId = U("cmt-789");
boost::optional<bool> viewed = true;
boost::optional<utility::string_t> type = U("reply");

api->getNotificationCount(tenantId, userId, urlId, fromCommentId, viewed, type)
    .then([](pplx::task<std::shared_ptr<GetNotificationCount_200_response>> task){
        try {
            auto resp = task.get();
            auto result = resp ? resp : std::make_shared<GetNotificationCount_200_response>();
        } catch (const std::exception&) {
        }
    });
[inline-code-end]

---
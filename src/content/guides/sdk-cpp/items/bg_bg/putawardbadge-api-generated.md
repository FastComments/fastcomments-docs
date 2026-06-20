## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| badgeId | string | Да |  |
| userId | string | Не |  |
| commentId | string | Не |  |
| broadcastId | string | Не |  |
| sso | string | Не |  |

## Отговор

Връща: [`AwardUserBadgeResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/AwardUserBadgeResponse.h)

## Пример

[inline-code-attrs-start title = 'Пример за putAwardBadge'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t badgeId = U("badge-elite-commenter");
boost::optional<utility::string_t> userId = boost::optional<utility::string_t>(U("jane.doe@example.com"));
boost::optional<utility::string_t> commentId = boost::optional<utility::string_t>(U("cmt-8f3a2b"));
boost::optional<utility::string_t> broadcastId;
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("my-tenant-123"));

api->putAwardBadge(badgeId, userId, commentId, broadcastId, sso)
.then([](pplx::task<std::shared_ptr<AwardUserBadgeResponse>> t){
    try {
        auto resp = t.get();
        auto out = resp ? resp : std::make_shared<AwardUserBadgeResponse>();
        std::cout << "Badge awarded successfully\n";
    } catch (const std::exception &e) {
        std::cerr << "Award failed: " << e.what() << '\n';
    }
}).wait();
[inline-code-end]

---
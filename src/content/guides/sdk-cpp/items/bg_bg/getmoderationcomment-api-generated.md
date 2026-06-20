## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| commentId | string | Да |  |
| includeEmail | bool | Не |  |
| includeIP | bool | Не |  |
| sso | string | Не |  |

## Отговор

Връща: [`ModerationAPICommentResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationAPICommentResponse.h)

## Пример

[inline-code-attrs-start title = 'Пример getModerationComment'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t commentId = U("cmt-987654");
boost::optional<bool> includeEmail = boost::optional<bool>(true);
boost::optional<bool> includeIP = boost::optional<bool>(false);
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));

api->getModerationComment(commentId, includeEmail, includeIP, sso)
.then([](std::shared_ptr<ModerationAPICommentResponse> resp) {
    auto result = resp ? resp : std::make_shared<ModerationAPICommentResponse>();
    if (resp)
        std::cout << "Comment fetched successfully" << std::endl;
    else
        std::cout << "No comment returned; using placeholder" << std::endl;
}).wait();
[inline-code-end]

---
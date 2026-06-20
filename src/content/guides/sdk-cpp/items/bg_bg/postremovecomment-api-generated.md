## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| commentId | string | Да |  |
| sso | string | Не |  |

## Отговор

Връща: [`PostRemoveCommentResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PostRemoveCommentResponse.h)

## Пример

[inline-code-attrs-start title = 'Пример за postRemoveComment'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t commentId = U("cmt-987654");
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));
api->postRemoveComment(commentId, sso)
    .then([](pplx::task<std::shared_ptr<PostRemoveCommentResponse>> t) {
        try {
            auto resp = t.get();
            auto result = resp ? resp : std::make_shared<PostRemoveCommentResponse>();
            if (result) std::cout << "Comment removed successfully\n";
        } catch (const std::exception &e) {
            std::cerr << "Remove failed: " << e.what() << "\n";
        }
    });
[inline-code-end]

---
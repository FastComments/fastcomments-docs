## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |
| blockFromCommentParams | BlockFromCommentParams | Да |  |
| userId | string | Не |  |
| anonUserId | string | Не |  |

## Отговор

Връща: [`BlockFromCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/BlockFromCommentPublic_200_response.h)

## Пример

[inline-code-attrs-start title = 'Пример за blockUserFromComment'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("cmt-456789");
auto blockParams = std::make_shared<BlockFromCommentParams>();
boost::optional<utility::string_t> userId = boost::optional<utility::string_t>(U("user@example.com"));
boost::optional<utility::string_t> anonUserId = boost::optional<utility::string_t>(U("anon-98765"));
api->blockUserFromComment(tenantId, commentId, *blockParams, userId, anonUserId)
.then([](std::shared_ptr<BlockFromCommentPublic_200_response> resp){
    if (resp) {
        std::cout << "User blocked successfully\n";
    } else {
        std::cout << "Block request returned empty response\n";
    }
}).wait();
[inline-code-end]

---
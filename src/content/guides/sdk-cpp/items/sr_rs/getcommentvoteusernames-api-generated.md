## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| commentId | string | Да |  |
| dir | int32_t | Да |  |
| sso | string | Не |  |

## Одговор

Враћа: [`GetCommentVoteUserNamesSuccessResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetCommentVoteUserNamesSuccessResponse.h)

## Пример

[inline-code-attrs-start title = 'getCommentVoteUserNames Пример'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("comment-456");
int32_t dir = 1;
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));
api->getCommentVoteUserNames(tenantId, commentId, dir, sso)
.then([](std::shared_ptr<GetCommentVoteUserNamesSuccessResponse> resp){
    auto result = resp ? resp : std::make_shared<GetCommentVoteUserNamesSuccessResponse>();
    std::cout << "Fetched comment vote user names" << std::endl;
});
[inline-code-end]

---
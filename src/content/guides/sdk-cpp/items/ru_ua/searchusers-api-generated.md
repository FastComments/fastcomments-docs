## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Да |  |
| usernameStartsWith | string | Нет |  |
| mentionGroupIds | vector<string | Нет |  |
| sso | string | Нет |  |
| searchSection | string | Нет |  |

## Ответ

Возвращает: [`SearchUsers_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/SearchUsers_200_response.h)

## Пример

[inline-code-attrs-start title = 'Пример searchUsers'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t urlId = U("/articles/2026/fast-comments-integration");
boost::optional<utility::string_t> usernameStartsWith = boost::optional<utility::string_t>(U("alex"));
std::vector<utility::string_t> groupsVec = { U("editors"), U("contributors") };
boost::optional<std::vector<utility::string_t>> mentionGroupIds = boost::optional<std::vector<utility::string_t>>(groupsVec);
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("sso-user-456"));
boost::optional<utility::string_t> searchSection = boost::optional<utility::string_t>(U("discussion"));

api->searchUsers(tenantId, urlId, usernameStartsWith, mentionGroupIds, sso, searchSection)
    .then([](pplx::task<std::shared_ptr<SearchUsers_200_response>> task) {
        try {
            auto resp = task.get();
            auto respCopy = std::make_shared<SearchUsers_200_response>(*resp);
            (void)respCopy;
        } catch (const std::exception&) {
        }
    });
[inline-code-end]

---
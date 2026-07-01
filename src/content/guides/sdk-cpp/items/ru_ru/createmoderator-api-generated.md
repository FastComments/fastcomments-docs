## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| createModeratorBody | CreateModeratorBody | Да |  |

## Ответ

Возвращает: [`CreateModeratorResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateModeratorResponse.h)

## Пример

[inline-code-attrs-start title = 'Пример createModerator'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
CreateModeratorBody moderatorBody;
moderatorBody.email = utility::conversions::to_string_t("moderator@example.com");
moderatorBody.name = utility::conversions::to_string_t("John Moderator");
moderatorBody.notes = boost::optional<utility::string_t>(utility::conversions::to_string_t("Community moderator"));
api->createModerator(utility::conversions::to_string_t("my-tenant-123"), moderatorBody)
    .then([](std::shared_ptr<CreateModeratorResponse> resp) {});
[inline-code-end]
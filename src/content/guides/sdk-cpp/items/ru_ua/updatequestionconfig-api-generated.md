## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |
| updateQuestionConfigBody | UpdateQuestionConfigBody | Да |  |

## Ответ

Возвращает: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Пример

[inline-code-attrs-start title = 'Пример updateQuestionConfig'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
auto configId = utility::string_t(U("question-config-456"));
auto updateBody = std::make_shared<UpdateQuestionConfigBody>();
updateBody->allowAnonymous = boost::optional<bool>(false);
updateBody->moderationRequired = boost::optional<bool>(true);
updateBody->defaultAssignee = boost::optional<utility::string_t>(U("moderator@example.com"));
api->updateQuestionConfig(tenantId, configId, *updateBody)
    .then([](pplx::task<std::shared_ptr<APIEmptyResponse>> t){
        try {
            auto resp = t.get();
        } catch (const std::exception&) {
        }
    });
[inline-code-end]
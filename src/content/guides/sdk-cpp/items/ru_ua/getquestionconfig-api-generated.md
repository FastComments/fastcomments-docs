## Параметры

| Имя | Тип | Обязательный | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |

## Ответ

Возвращает: [`GetQuestionConfig_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetQuestionConfig_200_response.h)

## Пример

[inline-code-attrs-start title = 'Пример getQuestionConfig'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t id = U("question-456");
boost::optional<utility::string_t> locale = U("en-US");
auto defaultResp = std::make_shared<GetQuestionConfig_200_response>();
api->getQuestionConfig(tenantId, id).then([defaultResp](pplx::task<std::shared_ptr<GetQuestionConfig_200_response>> t) {
    try {
        auto resp = t.get();
        if(!resp) resp = defaultResp;
        std::cout << "Question config retrieved" << std::endl;
    } catch(const std::exception &e) {
        std::cerr << "Failed to get question config: " << e.what() << std::endl;
    }
});
[inline-code-end]

---
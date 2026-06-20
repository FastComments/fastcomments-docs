## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Нет |  |
| createHashTagBody | CreateHashTagBody | Нет |  |

## Ответ

Возвращает: [`CreateHashTagResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateHashTagResponse.h)

## Пример

[inline-code-attrs-start title = 'Пример addHashTag'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = boost::optional<utility::string_t>(U("my-tenant-123"));
CreateHashTagBody createBody;
createBody.name = utility::string_t(U("release"));
createBody.createdBy = utility::string_t(U("admin@example.com"));
auto bodyOpt = boost::optional<CreateHashTagBody>(createBody);

api->addHashTag(tenantId, bodyOpt).then([](pplx::task<std::shared_ptr<CreateHashTagResponse>> t){
    try {
        auto resp = t.get();
        if (resp) {
            std::cout << "HashTag created successfully\n";
        } else {
            auto fallback = std::make_shared<CreateHashTagResponse>();
        }
    } catch (const std::exception &e) {
        std::cerr << "AddHashTag failed: " << e.what() << '\n';
    }
});
[inline-code-end]

---
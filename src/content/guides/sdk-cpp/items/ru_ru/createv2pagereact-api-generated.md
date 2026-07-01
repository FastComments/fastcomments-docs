## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|--------------|----------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| id | string | Yes |  |
| title | string | No |  |

## Ответ

Возвращает: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateV1PageReact.h)

## Пример

[inline-code-attrs-start title = 'createV2PageReact Пример'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
api->createV2PageReact(
    utility::string_t(U("my-tenant-789")),
    utility::string_t(U("https://example.com/articles/12345")),
    utility::string_t(U("user-42")),
    boost::optional<utility::string_t>(U("Helpful"))
).then([](pplx::task<std::shared_ptr<CreateV1PageReact>> task){
    try{
        auto response = task.get();
    }catch(const std::exception&){ }
});
[inline-code-end]
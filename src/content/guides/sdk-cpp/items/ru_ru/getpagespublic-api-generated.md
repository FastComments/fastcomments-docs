Выводит список страниц для тенанта. Используется десктоп-клиентом FChat для заполнения списка комнат.
Требует, чтобы `enableFChat` было равно true в резолвленном кастомном конфиге для каждой страницы.
Страницы, требующие SSO, фильтруются по групповому доступу запрашивающего пользователя.

## Параметры

| Имя | Тип | Обязательный | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| options | const GetPagesPublicOptions& | Да |  |

## Ответ

Возвращает: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetPublicPagesResponse.h)

## Пример

[inline-code-attrs-start title = 'Пример getPagesPublic'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
GetPagesPublicOptions options;
options.limit = boost::optional<int>(50);
options.cursor = boost::optional<utility::string_t>(U("cursor-token"));
api->getPagesPublic(tenantId, options).then([](pplx::task<std::shared_ptr<GetPublicPagesResponse>> task){
    try{
        auto response = task.get();
        // обработать ответ при необходимости
    }catch(const std::exception&){
        // обработать ошибку при необходимости
    }
});
[inline-code-end]
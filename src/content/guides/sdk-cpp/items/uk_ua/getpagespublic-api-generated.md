Список сторінок для тенанта. Використовується десктоп-клієнтом FChat для заповнення списку кімнат.
Вимагає, щоб `enableFChat` був встановлений у true в остаточній користувацькій конфігурації для кожної сторінки.
Сторінки, що вимагають SSO, фільтруються згідно з груповим доступом запитуваного користувача.

## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| cursor | string | Ні |  |
| limit | int32_t | Ні |  |
| q | string | Ні |  |
| sortBy | PagesSortBy | Ні |  |
| hasComments | bool | Ні |  |

## Відповідь

Повертає: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetPublicPagesResponse.h)

## Приклад

[inline-code-attrs-start title = 'Приклад використання getPagesPublic'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
boost::optional<utility::string_t> cursor = utility::string_t(U("cursor_abc"));
boost::optional<int32_t> limit = 50;
boost::optional<utility::string_t> q = utility::string_t(U("status:published"));
boost::optional<PagesSortBy> sortBy = PagesSortBy::NEWEST;
boost::optional<bool> hasComments = true;
api->getPagesPublic(tenantId, cursor, limit, q, sortBy, hasComments)
.then([](std::shared_ptr<GetPublicPagesResponse> resp){
    if (!resp) resp = std::make_shared<GetPublicPagesResponse>();
})
.wait();
[inline-code-end]

---
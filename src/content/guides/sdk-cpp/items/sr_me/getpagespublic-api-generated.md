Листа страница за tenant. Користи га FChat десктоп клијент за попуњавање своје листе соба. Захтева да `enableFChat` буде true у решеном прилагођеном конфигу за сваку страницу. Странице које захтевају SSO филтрирају се према приступу групе корисника који је послао захтев.

## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| cursor | string | Не |  |
| limit | int32_t | Не |  |
| q | string | Не |  |
| sortBy | PagesSortBy | Не |  |
| hasComments | bool | Не |  |

## Одговор

Враћа: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetPublicPagesResponse.h)

## Пример

[inline-code-attrs-start title = 'getPagesPublic Пример'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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
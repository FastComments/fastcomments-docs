Наводи странице за станара (tenant). Користи FChat десктоп клијент да попуни своју листу соба.
Захтева да `enableFChat` буде true на резолвованој прилагођеној конфигурацији за сваку страницу.
Странице које захтевају SSO се филтрирају у односу на приступ групама тражеог корисника.

## Параметри

| Назив | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| options | const GetPagesPublicOptions& | Да |  |

## Одговор

Враћа: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetPublicPagesResponse.h)

## Пример

[inline-code-attrs-start title = 'getPagesPublic пример'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
GetPagesPublicOptions options;
options.limit = boost::optional<int>(50);
options.cursor = boost::optional<utility::string_t>(U("cursor-token"));
api->getPagesPublic(tenantId, options).then([](pplx::task<std::shared_ptr<GetPublicPagesResponse>> task){
    try{
        auto response = task.get();
        // обради одговор ако је потребно
    }catch(const std::exception&){
        // обради грешку ако је потребно
    }
});
[inline-code-end]
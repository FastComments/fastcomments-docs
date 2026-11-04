Изброява страниците за даден tenant. Използва се от FChat десктоп клиента за попълване на списъка със стаи.
Изисква `enableFChat` да е true в резолвнатата персонализирана конфигурация за всяка страница.
Страниците, които изискват SSO, се филтрират спрямо груповия достъп на заявяващия потребител.

## Параметри

| Име | Тип | Задължителен | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| options | const GetPagesPublicOptions& | Да |  |

## Отговор

Връща: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetPublicPagesResponse.h)

## Пример

[inline-code-attrs-start title = 'Пример за getPagesPublic'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
GetPagesPublicOptions options;
options.limit = boost::optional<int>(50);
options.cursor = boost::optional<utility::string_t>(U("cursor-token"));
api->getPagesPublic(tenantId, options).then([](pplx::task<std::shared_ptr<GetPublicPagesResponse>> task){
    try{
        auto response = task.get();
        // обработи отговора, ако е необходимо
    }catch(const std::exception&){
        // обработи грешката, ако е необходимо
    }
});
[inline-code-end]
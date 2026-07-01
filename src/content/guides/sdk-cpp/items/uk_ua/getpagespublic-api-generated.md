List pages for a tenant. Used by the FChat desktop client to populate its room list.  
Вимагає, щоб `enableFChat` було встановлено в true в розвʼязаній кастомній конфігурації для кожної сторінки.  
Сторінки, які вимагають SSO, фільтруються відповідно до групового доступу запитуючого користувача.

## Parameters

| Назва | Тип | Обов’язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| options | const GetPagesPublicOptions& | Так |  |

## Відповідь

Returns: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetPublicPagesResponse.h)

## Приклад

[inline-code-attrs-start title = 'Приклад getPagesPublic'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
GetPagesPublicOptions options;
options.limit = boost::optional<int>(50);
options.cursor = boost::optional<utility::string_t>(U("cursor-token"));
api->getPagesPublic(tenantId, options).then([](pplx::task<std::shared_ptr<GetPublicPagesResponse>> task){
    try{
        auto response = task.get();
        // обробити відповідь, якщо потрібно
    }catch(const std::exception&){
        // обробити помилку, якщо потрібно
    }
});
[inline-code-end]

---
## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| options | const GetCountOptions& | Так |  |

## Відповідь

Повертає: [`ModerationAPICountCommentsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationAPICountCommentsResponse.h)

## Приклад

[inline-code-attrs-start title = 'Приклад getCount'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
GetCountOptions options;
options.userEmail = boost::optional<utility::string_t>(U("user@example.com"));
options.maxResults = boost::optional<int>(100);
api->getCount(tenantId, options).then([](pplx::task<std::shared_ptr<ModerationAPICountCommentsResponse>> task){
    try{
        auto resp = task.get();
        auto copy = std::make_shared<ModerationAPICountCommentsResponse>(*resp);
        std::cout << "Count: " << copy->count << std::endl;
    }catch(const std::exception& e){
        std::cerr << "Error: " << e.what() << std::endl;
    }
});
[inline-code-end]
## Параметри

| Назва | Тип | Обов’язково | Опис |
|------|------|--------------|------|
| tenantId | string | Так |  |
| page | double | Ні |  |

## Відповідь

Повертає: [`GetHashTagsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetHashTagsResponse.h)

## Приклад

[inline-code-attrs-start title = 'Приклад getHashTags'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
boost::optional<double> page = 2.0;

api->getHashTags(tenantId, page).then([](pplx::task<std::shared_ptr<GetHashTagsResponse>> task) {
    try {
        auto resultPtr = task.get();
        auto response = std::make_shared<GetHashTagsResponse>(*resultPtr);
        // використати response
    } catch (const std::exception&) {
        // обробити помилку
    }
});
[inline-code-end]
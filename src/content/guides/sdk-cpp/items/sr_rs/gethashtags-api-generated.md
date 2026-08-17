## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| page | double | Не |  |

## Одговор

Враћа: [`GetHashTagsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetHashTagsResponse.h)

## Пример

[inline-code-attrs-start title = 'getHashTags Primer'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
boost::optional<double> page = 2.0;

api->getHashTags(tenantId, page).then([](pplx::task<std::shared_ptr<GetHashTagsResponse>> task) {
    try {
        auto resultPtr = task.get();
        auto response = std::make_shared<GetHashTagsResponse>(*resultPtr);
        // користи одговор
    } catch (const std::exception&) {
        // обради грешку
    }
});
[inline-code-end]

---
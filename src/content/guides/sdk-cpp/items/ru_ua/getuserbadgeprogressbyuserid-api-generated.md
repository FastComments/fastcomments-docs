## Параметри

| Назва | Тип | Обов’язковий | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| userId | string | Так |  |

## Відповідь

Повертає: [`APIGetUserBadgeProgressResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIGetUserBadgeProgressResponse.h)

## Приклад

[inline-code-attrs-start title = 'Приклад getUserBadgeProgressByUserId'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<std::shared_ptr<APIGetUserBadgeProgressResponse>> responseOpt;
api->getUserBadgeProgressByUserId(
    utility::conversions::to_string_t("my-tenant-123"),
    utility::conversions::to_string_t("user@example.com"))
    .then([&responseOpt](pplx::task<std::shared_ptr<APIGetUserBadgeProgressResponse>> t) {
        try {
            responseOpt = t.get();
        } catch (...) {
            responseOpt = boost::none;
        }
    });
[inline-code-end]

---
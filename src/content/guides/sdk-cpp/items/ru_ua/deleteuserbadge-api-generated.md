## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Відповідь

Повертає: [`APIEmptySuccessResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptySuccessResponse.h)

## Приклад

[inline-code-attrs-start title = 'Приклад deleteUserBadge'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = boost::optional<utility::string_t>(U("my-tenant-123"));
auto badgeId = utility::string_t(U("badge-789"));
api->deleteUserBadge(tenantId.value(), badgeId)
    .then([](std::shared_ptr<APIEmptySuccessResponse> resp){
        auto copy = std::make_shared<APIEmptySuccessResponse>(*resp);
    })
    .then([](pplx::task<void> t){
        try{ t.get(); } catch(const std::exception&){ }
    });
[inline-code-end]
## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|------------|------|
| tenantId | string | Yes |  |
| sso | string | No |  |

## Відповідь

Повертає: [`GetBannedUsersCountResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetBannedUsersCountResponse.h)

## Приклад

[inline-code-attrs-start title = 'Приклад getCounts'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
api->getCounts(
    utility::conversions::to_string_t("my-tenant-123"),
    boost::optional<utility::string_t>(utility::conversions::to_string_t("john.doe@example.com"))
).then([](pplx::task<std::shared_ptr<GetBannedUsersCountResponse>> t){
    try{
        auto response = t.get();
    }catch(const std::exception&){
    }
});
[inline-code-end]
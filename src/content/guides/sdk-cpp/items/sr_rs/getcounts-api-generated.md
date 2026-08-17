## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| sso | string | Не |  |

## Одговор

Враћа: [`GetBannedUsersCountResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetBannedUsersCountResponse.h)

## Пример

[inline-code-attrs-start title = 'getCounts Пример'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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

---
## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Да |  |
| id | string | Да |  |

## Одговор

Враћа: [`GetV2PageReactUsersResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetV2PageReactUsersResponse.h)

## Пример

[inline-code-attrs-start title = 'getV2PageReactUsers Primer'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto urlId = utility::conversions::to_string_t("page-456");
auto id = utility::conversions::to_string_t("react-789");
boost::optional<utility::string_t> maybeFilter;
api->getV2PageReactUsers(tenantId, urlId, id).then([](pplx::task<std::shared_ptr<GetV2PageReactUsersResponse>> task){
    try{
        auto response = task.get();
        // Користите одговор по потреби
    }catch(const std::exception&){
        // Обрадите грешку
    }
});
[inline-code-end]
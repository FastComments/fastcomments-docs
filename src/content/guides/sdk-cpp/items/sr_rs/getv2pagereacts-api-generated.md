## Параметри

| Назив | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Да |  |

## Одговор

Враћа: [`GetV2PageReacts`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetV2PageReacts.h)

## Пример

[inline-code-attrs-start title = 'Пример getV2PageReacts'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = utility::conversions::to_string_t("my-tenant-123");
utility::string_t urlId = utility::conversions::to_string_t("page-55f3b2");
boost::optional<utility::string_t> maybeFilter = utility::conversions::to_string_t("recent");
auto fallback = std::make_shared<GetV2PageReacts>();
api->getV2PageReacts(tenantId, urlId)
.then([fallback](std::shared_ptr<GetV2PageReacts> resp){
    auto result = resp ? resp : fallback;
    std::cout << "Received reacts object at " << result.get() << std::endl;
});
[inline-code-end]
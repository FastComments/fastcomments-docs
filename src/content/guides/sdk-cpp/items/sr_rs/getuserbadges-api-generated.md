## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| userId | string | Не |  |
| badgeId | string | Не |  |
| type | double | Не |  |
| displayedOnComments | bool | Не |  |
| limit | double | Не |  |
| skip | double | Не |  |

## Одговор

Враћа: [`GetUserBadges_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetUserBadges_200_response.h)

## Пример

[inline-code-attrs-start title = 'Пример getUserBadges'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = utility::conversions::to_string_t("my-tenant-123");
boost::optional<utility::string_t> userId(utility::conversions::to_string_t("user@example.com"));
boost::optional<utility::string_t> badgeId(utility::conversions::to_string_t("badge-elite"));
boost::optional<double> type(1.0);
boost::optional<bool> displayedOnComments(true);
boost::optional<double> limit(50.0);
boost::optional<double> skip(0.0);
auto resultHolder = std::make_shared<GetUserBadges_200_response>();
api->getUserBadges(tenantId, userId, badgeId, type, displayedOnComments, limit, skip)
.then([&resultHolder](std::shared_ptr<GetUserBadges_200_response> resp){
    if (resp) resultHolder = resp;
    return resultHolder;
});
[inline-code-end]

---
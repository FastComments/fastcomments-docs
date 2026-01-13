## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| postIds | vector<string | Не |  |
| sso | string | Не |  |

## Одговор

Враћа: [`GetUserReactsPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetUserReactsPublic_200_response.h)

## Пример

[inline-code-attrs-start title = 'getUserReactsPublic Пример'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = utility::conversions::to_string_t("my-tenant-123");
std::vector<utility::string_t> postIdsVec = {
    utility::conversions::to_string_t("post-456"),
    utility::conversions::to_string_t("post-789")
};
boost::optional<std::vector<utility::string_t>> postIdsOpt = postIdsVec;
boost::optional<utility::string_t> ssoOpt = utility::conversions::to_string_t("user@example.com");
api->getUserReactsPublic(tenantId, postIdsOpt, ssoOpt)
    .then([](pplx::task<std::shared_ptr<GetUserReactsPublic_200_response>> t) {
        try {
            auto resp = t.get();
            if (!resp) resp = std::make_shared<GetUserReactsPublic_200_response>();
        } catch (const std::exception&) {
        }
    });
[inline-code-end]

---
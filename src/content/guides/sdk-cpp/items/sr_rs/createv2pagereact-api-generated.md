## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Да |  |
| id | string | Да |  |
| title | string | Не |  |

## Одговор

Враћа: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateV1PageReact.h)

## Пример

[inline-code-attrs-start title = 'Пример createV2PageReact'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = utility::conversions::to_string_t("my-tenant-123");
utility::string_t urlId = utility::conversions::to_string_t("homepage-landing");
utility::string_t id = utility::conversions::to_string_t("user-987@example.com");
boost::optional<utility::string_t> title = boost::optional<utility::string_t>(utility::conversions::to_string_t("Introducing FastComments"));

api->createV2PageReact(tenantId, urlId, id, title)
    .then([](std::shared_ptr<CreateV1PageReact> resp){
        auto createdCopy = std::make_shared<CreateV1PageReact>(*resp);
        return createdCopy;
    })
    .then([](std::shared_ptr<CreateV1PageReact> created){
        (void)created;
    })
    .wait();
[inline-code-end]

---
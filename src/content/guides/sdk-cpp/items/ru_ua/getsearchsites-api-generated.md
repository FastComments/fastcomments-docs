## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| value | string | Нет |  |
| sso | string | Нет |  |

## Ответ

Возвращает: [`ModerationSiteSearchResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationSiteSearchResponse.h)

## Пример

[inline-code-attrs-start title = 'Пример getSearchSites'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> valueOpt = boost::optional<utility::string_t>(U("my-tenant-123"));
boost::optional<utility::string_t> ssoOpt = boost::optional<utility::string_t>(U("user@example.com"));
api->getSearchSites(valueOpt, ssoOpt)
    .then([](std::shared_ptr<ModerationSiteSearchResponse> resp){
        auto response = resp ? resp : std::make_shared<ModerationSiteSearchResponse>();
        (void)response;
    })
    .wait();
[inline-code-end]
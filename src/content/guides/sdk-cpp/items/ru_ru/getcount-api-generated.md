## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| textSearch | string | Нет |  |
| byIPFromComment | string | Нет |  |
| filter | string | Нет |  |
| searchFilters | string | Нет |  |
| demo | bool | Нет |  |
| sso | string | Нет |  |

## Ответ

Возвращает: [`ModerationAPICountCommentsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationAPICountCommentsResponse.h)

## Пример

[inline-code-attrs-start title = 'Пример getCount'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t text = U("abusive language");
utility::string_t ip = U("203.0.113.45");
utility::string_t filter = U("status:flagged");
utility::string_t searchFilters = U("platform:mobile");
utility::string_t sso = U("admin@my-tenant-123.com");
auto textOpt = boost::optional<utility::string_t>(text);
auto ipOpt = boost::optional<utility::string_t>(ip);
auto filterOpt = boost::optional<utility::string_t>(filter);
auto searchFiltersOpt = boost::optional<utility::string_t>(searchFilters);
auto demoOpt = boost::optional<bool>(true);
auto ssoOpt = boost::optional<utility::string_t>(sso);
api->getCount(textOpt, ipOpt, filterOpt, searchFiltersOpt, demoOpt, ssoOpt)
.then([](pplx::task<std::shared_ptr<ModerationAPICountCommentsResponse>> t){
    try {
        auto resp = t.get();
        auto finalResp = resp ? resp : std::make_shared<ModerationAPICountCommentsResponse>();
        (void)finalResp;
    } catch (...) {}
});
[inline-code-end]

---
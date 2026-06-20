---
Сводная информация о пользователях для тенанта. По заданным userIds возвращает отображаемую информацию из User / SSOUser.
Используется виджетом комментариев для обогащения пользователей, которые только что появились через событие присутствия.
Нет контекста страницы: конфиденциальность соблюдается одинаково (приватные профили маскируются).

## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| ids | string | Да |  |

## Ответ

Возвращает: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PageUsersInfoResponse.h)

## Пример

[inline-code-attrs-start title = 'Пример getUsersInfo'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t ids = U("alice@example.com,bob@example.com");
boost::optional<utility::string_t> statusFilter = U("active");
api->getUsersInfo(tenantId, ids).then([](pplx::task<std::shared_ptr<PageUsersInfoResponse>> t){
    try {
        auto res = t.get();
        if (res) {
            auto responseCopy = std::make_shared<PageUsersInfoResponse>(*res);
        }
    } catch (const std::exception&) {}
});
[inline-code-end]

---
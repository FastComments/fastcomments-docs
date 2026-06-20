---
Масовне информације о корисницима за тенанта. За дате userIds, враћа приказне информације из User / SSOUser.
Користи га видгет за коментаре да обогати кориснике који су се управо појавили путем догађаја присутности.
Нема контекста странице: приватност се доследно примењује (приватни профили су сакривени).

## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| ids | string | Да |  |

## Одговор

Враћа: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PageUsersInfoResponse.h)

## Пример

[inline-code-attrs-start title = 'getUsersInfo Пример'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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
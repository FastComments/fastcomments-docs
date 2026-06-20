---
Претходни коментатори на страници који тренутно нису на мрежи. Сортирано по displayName.
Користите ово након исцрпљивања /users/online да бисте приказали "Чланови" одељак.
Курсорска пагинација по commenterName: сервер пролази делимични {tenantId, urlId, commenterName}
индекс од afterName унапред коришћењем $gt, без трошка $skip.

## Параметри

| Име | Тип | Потребно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Да |  |
| afterName | string | Не |  |
| afterUserId | string | Не |  |

## Одговор

Враћа: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PageUsersOfflineResponse.h)

## Пример

[inline-code-attrs-start title = 'Пример getOfflineUsers'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
auto urlId = utility::string_t(U("article-456"));
boost::optional<utility::string_t> afterName = boost::optional<utility::string_t>(U("jane.doe@example.com"));
boost::optional<utility::string_t> afterUserId = boost::optional<utility::string_t>(U("user-789"));
api->getOfflineUsers(tenantId, urlId, afterName, afterUserId).then([](std::shared_ptr<PageUsersOfflineResponse> resp){
    auto result = resp ? resp : std::make_shared<PageUsersOfflineResponse>();
    (void)result;
});
[inline-code-end]

---
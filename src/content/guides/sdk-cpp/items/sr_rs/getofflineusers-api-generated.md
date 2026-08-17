Претходни коментатори на страници који ТРЕНУТНО нису онлајн. Сортирани по displayName.  
Користите ово након што исцрпите /users/online да прикажете одељак „Чланови“.  
Курсорска пагинација на commenterName: сервер пролази кроз парцијални {tenantId, urlId, commenterName} индекс од afterName напред помоћу $gt, без $skip трошкова.

## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| options | const GetOfflineUsersOptions& | Yes |  |

## Одговор

Враћа: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/PageUsersOfflineResponse.h)

## Пример

[inline-code-attrs-start title = 'Primer getOfflineUsers'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t urlId = U("page-456");
GetOfflineUsersOptions options;
options.limit = boost::optional<int>(50);
options.includeDetails = boost::optional<bool>(true);

api->getOfflineUsers(tenantId, urlId, options)
    .then([](pplx::task<std::shared_ptr<PageUsersOfflineResponse>> t) {
        try {
            auto response = t.get();
        } catch (const std::exception&) {
        }
    });
[inline-code-end]
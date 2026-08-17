Тренутно онлајн посматрачи странице: људи чија вебсокет сесија је претплаћена на страницу тренутно.  
Враћа anonCount + totalCount (претплатници у целој соби, укључујући анонимне посматраче које не набрајамо).

## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| options | const GetOnlineUsersOptions& | Yes |  |

## Одговор

Враћа: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/PageUsersOnlineResponse.h)

## Пример

[inline-code-attrs-start title = 'Пример getOnlineUsers'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t urlId = U("article-456");
auto options = std::make_shared<GetOnlineUsersOptions>();
options->maxResults = boost::optional<int>(100);
options->includeInactive = boost::optional<bool>(false);
api->getOnlineUsers(tenantId, urlId, *options).then([](pplx::task<std::shared_ptr<PageUsersOnlineResponse>> task){
    try{
        auto response = task.get();
    }catch(const std::exception&){
    }
});
[inline-code-end]
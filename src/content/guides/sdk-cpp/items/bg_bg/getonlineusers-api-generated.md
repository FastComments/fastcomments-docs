Текущо онлайн зрители на страница: хора, чиято уебсокет сесия е абонирана за страницата в момента.  
Връща anonCount + totalCount (абонати в цялата стая, включително анонимни зрители, които не изброяваме).

## Parameters

| Име | Тип | Задължително | Описание |
|------|------|--------------|----------|
| tenantId | string | Да |  |
| urlId | string | Да |  |
| options | const GetOnlineUsersOptions& | Да |  |

## Response

Връща: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PageUsersOnlineResponse.h)

## Пример

[inline-code-attrs-start title = 'Пример за getOnlineUsers'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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
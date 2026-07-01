目前線上觀看頁面的使用者：目前其 websocket session 已訂閱該頁面的使用者。  
返回 anonCount + totalCount（全域訂閱者，包含我們未列舉的匿名觀看者）。

## 參數

| 名稱 | 型別 | 必填 | 說明 |
|------|------|------|------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |
| options | const GetOnlineUsersOptions& | 是 |  |

## 回應

返回: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PageUsersOnlineResponse.h)

## 範例

[inline-code-attrs-start title = 'getOnlineUsers 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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
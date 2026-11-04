頁面上過去曾留言但目前不在線上的使用者。按 displayName 排序。
在用完 /users/online 之後,使用此 API 來渲染「成員」區塊。
基於 commenterName 的游標分頁:伺服器透過 $gt 從 afterName 開始,沿著部分索引 {tenantId, urlId, commenterName} 向前走,沒有 $skip 的成本。

## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |
| options | const GetOfflineUsersOptions& | 是 |  |

## 回應

回傳:[`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/PageUsersOfflineResponse.h)

## 範例

[inline-code-attrs-start title = 'getOfflineUsers 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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
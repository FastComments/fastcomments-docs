## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| options | const GetSearchUsersOptions& | 是 |  |

## 回應

返回: [`ModerationUserSearchResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationUserSearchResponse.h)

## 範例

[inline-code-attrs-start title = 'getSearchUsers 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
GetSearchUsersOptions opts;
opts.query = utility::string_t(U("john.doe@example.com"));
opts.page = boost::optional<int>(1);
opts.pageSize = boost::optional<int>(20);

api->getSearchUsers(tenantId, opts)
    .then([](std::shared_ptr<ModerationUserSearchResponse> resp) {
        for (const auto& user : resp->users) {
            std::wcout << user.id << L" - " << user.email << std::endl;
        }
    });
[inline-code-end]
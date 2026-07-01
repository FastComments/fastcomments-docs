## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| options | const GetUserBadgeProgressListOptions& | Yes |  |

## レスポンス

戻り値: [`APIGetUserBadgeProgressListResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIGetUserBadgeProgressListResponse.h)

## 例

[inline-code-attrs-start title = 'getUserBadgeProgressList の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
GetUserBadgeProgressListOptions options;
options.userId = U("user@example.com");
options.page = boost::optional<int>(1);
options.pageSize = boost::optional<int>(20);
api->getUserBadgeProgressList(tenantId, options)
    .then([](std::shared_ptr<APIGetUserBadgeProgressListResponse> resp) {
        if (!resp) return;
        for (const auto& badge : resp->badges) {
            // バッジを処理する
        }
    });
[inline-code-end]
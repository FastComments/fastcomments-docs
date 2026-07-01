過去にページにコメントしたが、現在オンラインではないユーザー。displayName でソートされます。  
`/users/online` をすべて使用した後に、"Members" セクションを表示するために使用します。  
commenterName に対するカーソルページング: サーバーは部分的な `{tenantId, urlId, commenterName}` インデックスを afterName 以降へ $gt で進め、$skip コストはかかりません。

## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| urlId | string | はい |  |
| options | const GetOfflineUsersOptions& | はい |  |

## レスポンス

返却: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PageUsersOfflineResponse.h)

## 例

[inline-code-attrs-start title = 'getOfflineUsers の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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
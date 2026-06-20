テナントのユーザー情報を一括取得します。userIds を指定すると、User / SSOUser から表示用情報を返します。
コメントウィジェットが、presence イベントで新たに出現したユーザーの情報を補完するために使用します。
ページコンテキストがない場合：プライバシーは一律に適用されます（非公開プロファイルはマスクされます）。

## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| ids | string | Yes |  |

## レスポンス

返却: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PageUsersInfoResponse.h)

## 例

[inline-code-attrs-start title = 'getUsersInfo の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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
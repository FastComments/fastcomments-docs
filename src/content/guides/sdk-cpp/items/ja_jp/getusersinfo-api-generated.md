テナントのユーザー情報を一括取得します。userId を指定すると、User / SSOUser から表示情報を返します。
プレゼンスイベント経由で新たに現れたユーザーをコメントウィジェットで補完するために使用されます。
ページコンテキストはありません: プライバシーは一律に適用されます(非公開プロフィールはマスクされます)。

## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| ids | string | はい |  |

## レスポンス

戻り値: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/PageUsersInfoResponse.h)

## 例

[inline-code-attrs-start title = 'getUsersInfo の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t ids = U("alice@example.com,bob@example.com");
boost::optional<utility::string_t> locale = boost::make_optional(U("en-US"));

api->getUsersInfo(tenantId, ids).then([](pplx::task<std::shared_ptr<PageUsersInfoResponse>> t){
    try{
        auto response = t.get();
        // レスポンスを処理
    }catch(const std::exception&){
        // エラーを処理
    }
});
[inline-code-end]
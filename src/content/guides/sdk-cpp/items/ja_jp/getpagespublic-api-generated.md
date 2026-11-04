テナントのページ一覧を取得します。FChat デスクトップクライアントがルーム一覧を表示するために使用します。
各ページの解決されたカスタム設定で `enableFChat` が true である必要があります。
SSO が必要なページは、リクエストしたユーザーのグループアクセス権に基づいてフィルタリングされます。

## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| options | const GetPagesPublicOptions& | はい |  |

## レスポンス

戻り値: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetPublicPagesResponse.h)

## 例

[inline-code-attrs-start title = 'getPagesPublic の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
GetPagesPublicOptions options;
options.limit = boost::optional<int>(50);
options.cursor = boost::optional<utility::string_t>(U("cursor-token"));
api->getPagesPublic(tenantId, options).then([](pplx::task<std::shared_ptr<GetPublicPagesResponse>> task){
    try{
        auto response = task.get();
        // 必要に応じてレスポンスを処理
    }catch(const std::exception&){
        // 必要に応じてエラーを処理
    }
});
[inline-code-end]
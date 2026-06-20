## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tag | string | はい |  |
| tenantId | string | いいえ |  |
| deleteHashTagRequestBody | DeleteHashTagRequestBody | いいえ |  |

## レスポンス

戻り値: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 例

[inline-code-attrs-start title = 'deleteHashTag の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tag = U("launch");
utility::string_t tenantId = U("my-tenant-123");
DeleteHashTagRequestBody body;
boost::optional<utility::string_t> optTenant(tenantId);
boost::optional<DeleteHashTagRequestBody> optBody(body);
api->deleteHashTag(tag, optTenant, optBody)
.then([](pplx::task<std::shared_ptr<APIEmptyResponse>> task){
    try {
        auto resp = task.get();
        if (resp) {
            auto copied = std::make_shared<APIEmptyResponse>(*resp);
            (void)copied;
        }
    } catch (const std::exception &e) {
        (void)e;
    }
});
[inline-code-end]

---
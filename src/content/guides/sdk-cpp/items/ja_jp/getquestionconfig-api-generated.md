## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |

## レスポンス

戻り値: [`GetQuestionConfigResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetQuestionConfigResponse.h)

## 例

[inline-code-attrs-start title = 'getQuestionConfig の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t questionId = U("qstn-456");
boost::optional<utility::string_t> ifNoneMatch = U("W/\"5a2f3c\"");
api->getQuestionConfig(tenantId, questionId)
.then([ifNoneMatch](pplx::task<std::shared_ptr<GetQuestionConfigResponse>> t){
    try {
        auto resp = t.get();
        if (ifNoneMatch) {
            auto etag = *ifNoneMatch;
            (void)etag;
        }
        auto cfg = resp ? resp : std::make_shared<GetQuestionConfigResponse>();
        (void)cfg;
    } catch (const std::exception &ex) {
        (void)ex;
    }
});
[inline-code-end]

---
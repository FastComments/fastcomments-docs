## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## レスポンス

返却: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 例

[inline-code-attrs-start title = 'deleteQuestionConfig の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto configId = utility::conversions::to_string_t("question-config-456");

api->deleteQuestionConfig(tenantId, configId)
    .then([](std::shared_ptr<APIEmptyResponse> resp) {
        // 削除成功を処理
    })
    .then([](pplx::task<void> t) {
        try {
            t.get();
        } catch (const std::exception&) {
            // エラーを処理
        }
    });
[inline-code-end]

---
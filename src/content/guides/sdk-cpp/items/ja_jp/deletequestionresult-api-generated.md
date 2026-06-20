## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |

## レスポンス

戻り値: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 例

[inline-code-attrs-start title = 'deleteQuestionResult の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
auto questionId = utility::string_t(U("qst-456789"));
boost::optional<utility::string_t> operatorNote = boost::optional<utility::string_t>(U("admin-request-001"));

api->deleteQuestionResult(tenantId, questionId)
    .then([operatorNote](pplx::task<std::shared_ptr<APIEmptyResponse>> t) {
        try {
            auto resp = t.get();
            auto result = resp ? resp : std::make_shared<APIEmptyResponse>();
            if (operatorNote) std::cout << "Deleted (note): " << utility::conversions::to_utf8string(*operatorNote) << std::endl;
            else std::cout << "Deleted" << std::endl;
            return result;
        } catch (const std::exception& e) {
            std::cerr << "Delete failed: " << e.what() << std::endl;
            return std::shared_ptr<APIEmptyResponse>(nullptr);
        }
    });
[inline-code-end]

---
## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |

## 응답

반환: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 예제

[inline-code-attrs-start title = 'deleteQuestionConfig 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t id = U("question-456");
boost::optional<utility::string_t> correlationId = boost::optional<utility::string_t>(U("corr-12345"));
auto placeholder = std::make_shared<APIEmptyResponse>();
api->deleteQuestionConfig(tenantId, id)
.then([](pplx::task<std::shared_ptr<APIEmptyResponse>> task)
{
    try {
        auto resp = task.get();
        if (resp) std::cout << "Question config deleted\n";
        else std::cout << "No response body\n";
    } catch (const std::exception &e) {
        std::cerr << "Delete failed: " << e.what() << '\n';
    }
});
[inline-code-end]

---
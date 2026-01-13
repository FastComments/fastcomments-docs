## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |

## 응답

반환: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## 예제

[inline-code-attrs-start title = 'deleteQuestionResult 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> maybeTenant = boost::none;
utility::string_t defaultTenant = U("my-tenant-123");
utility::string_t questionId = U("question-456");
utility::string_t tenant = maybeTenant ? *maybeTenant : defaultTenant;
api->deleteQuestionResult(tenant, questionId)
.then([](pplx::task<std::shared_ptr<FlagCommentPublic_200_response>> task){
    try {
        auto resp = task.get();
        auto processed = std::make_shared<FlagCommentPublic_200_response>(*resp);
    } catch (const std::exception&) {
    }
});
[inline-code-end]

---
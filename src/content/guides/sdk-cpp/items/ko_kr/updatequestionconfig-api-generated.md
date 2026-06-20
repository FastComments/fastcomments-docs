## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| updateQuestionConfigBody | UpdateQuestionConfigBody | 예 |  |

## 응답

반환: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 예제

[inline-code-attrs-start title = 'updateQuestionConfig 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
auto configId = utility::string_t(U("question-config-456"));
auto updateBody = std::make_shared<UpdateQuestionConfigBody>();
updateBody->allowAnonymous = boost::optional<bool>(false);
updateBody->moderationRequired = boost::optional<bool>(true);
updateBody->defaultAssignee = boost::optional<utility::string_t>(U("moderator@example.com"));
api->updateQuestionConfig(tenantId, configId, *updateBody)
    .then([](pplx::task<std::shared_ptr<APIEmptyResponse>> t){
        try {
            auto resp = t.get();
        } catch (const std::exception&) {
        }
    });
[inline-code-end]

---
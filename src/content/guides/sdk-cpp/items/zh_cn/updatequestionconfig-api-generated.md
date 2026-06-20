---
## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| updateQuestionConfigBody | UpdateQuestionConfigBody | 是 |  |

## 响应

返回： [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 示例

[inline-code-attrs-start title = 'updateQuestionConfig 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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
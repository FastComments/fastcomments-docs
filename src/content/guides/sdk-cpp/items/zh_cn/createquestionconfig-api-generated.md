## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createQuestionConfigBody | CreateQuestionConfigBody | Yes |  |

## 响应

返回: [`CreateQuestionConfig_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateQuestionConfig_200_response.h)

## 示例

[inline-code-attrs-start title = 'createQuestionConfig 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
CreateQuestionConfigBody body;
body.name = utility::string_t(U("Feature X Feedback"));
body.description = boost::optional<utility::string_t>(utility::string_t(U("Ask users if Feature X met their needs")));
body.createdBy = utility::string_t(U("admin@mycompany.com"));
body.enabled = boost::optional<bool>(true);
api->createQuestionConfig(tenantId, body).then([](std::shared_ptr<CreateQuestionConfig_200_response> resp){
    if(resp){
        auto result = std::make_shared<CreateQuestionConfig_200_response>(*resp);
        std::wcout << U("Question config created successfully") << std::endl;
    } else {
        std::wcout << U("Failed to create question config") << std::endl;
    }
});
[inline-code-end]

---
## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| createQuestionConfigBody | CreateQuestionConfigBody | כן |  |

## תגובה

מחזיר: [`CreateQuestionConfigResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateQuestionConfigResponse.h)

## דוגמה

[inline-code-attrs-start title = 'createQuestionConfig דוגמה'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
CreateQuestionConfigBody configBody;
configBody.question = utility::string_t(U("How satisfied are you with our service?"));
configBody.required = true;
configBody.defaultAnswer = boost::optional<utility::string_t>(utility::string_t(U("Very satisfied")));
api->createQuestionConfig(tenantId, configBody).then([](pplx::task<std::shared_ptr<CreateQuestionConfigResponse>> task){
    try{
        auto response = task.get();
    }catch(const std::exception&){}
});
[inline-code-end]
## פרמטרים

| שם | סוג | חובה | תיאור |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateQuestionConfigBody | UpdateQuestionConfigBody | Yes |  |

## תגובה

מחזיר: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## דוגמה

[inline-code-attrs-start title = 'דוגמת updateQuestionConfig'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
UpdateQuestionConfigBody updateBody;
updateBody.enabled = true;
updateBody.maxResponses = boost::optional<int>{10};
updateBody.notes = boost::optional<utility::string_t>{U("Config updated via SDK")};

api->updateQuestionConfig(U("my-tenant-123"), U("config-789"), updateBody)
    .then([](std::shared_ptr<APIEmptyResponse> resp) {
    })
    .then([](pplx::task<void> t) {
        try { t.get(); } catch (const std::exception&) {}
    });
[inline-code-end]
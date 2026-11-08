## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| id | string | हाँ |  |
| updateQuestionConfigBody | UpdateQuestionConfigBody | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'updateQuestionConfig उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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
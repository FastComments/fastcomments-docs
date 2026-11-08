## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| options | const GetApiIdsOptions& | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`ModerationAPIGetCommentIdsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/ModerationAPIGetCommentIdsResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'getApiIds उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
GetApiIdsOptions options;
options.limit = boost::optional<int>(100);
options.cursor = boost::optional<utility::string_t>(U("next-page-token"));
api->getApiIds(tenantId, options).then([](pplx::task<std::shared_ptr<ModerationAPIGetCommentIdsResponse>> t){
    try{
        auto response = t.get();
        auto ids = response->commentIds;
    }catch(const std::exception&){
    }
});
[inline-code-end]
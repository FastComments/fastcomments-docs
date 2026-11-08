## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| urlId | string | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/CreateV1PageReact.h)

## उदाहरण

[inline-code-attrs-start title = 'deleteV1PageReact उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto urlId = utility::conversions::to_string_t("page-abc-456");
api->deleteV1PageReact(tenantId, urlId).then([](pplx::task<std::shared_ptr<CreateV1PageReact>> task){
    try{
        auto response = task.get();
        boost::optional<std::shared_ptr<CreateV1PageReact>> optResponse = response;
    }catch(...){
    }
});
[inline-code-end]
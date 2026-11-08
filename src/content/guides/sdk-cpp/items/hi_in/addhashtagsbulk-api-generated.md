## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| bulkCreateHashTagsBody | BulkCreateHashTagsBody | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`BulkCreateHashTagsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/BulkCreateHashTagsResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'addHashTagsBulk उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
BulkCreateHashTagsBody bulkBody;
bulkBody.tags = { utility::string_t(U("announcement")), utility::string_t(U("feature-request")) };
bulkBody.description = boost::optional<utility::string_t>(U("Bulk tag creation"));
api->addHashTagsBulk(tenantId, bulkBody).then([](pplx::task<std::shared_ptr<BulkCreateHashTagsResponse>> task){
    try{
        auto resp = task.get();
    }catch(const std::exception&){
    }
});
[inline-code-end]
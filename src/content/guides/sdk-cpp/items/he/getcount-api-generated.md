## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| options | const GetCountOptions& | כן |  |

## תגובה

מחזיר: [`ModerationAPICountCommentsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationAPICountCommentsResponse.h)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getCount'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
GetCountOptions options;
options.userEmail = boost::optional<utility::string_t>(U("user@example.com"));
options.maxResults = boost::optional<int>(100);
api->getCount(tenantId, options).then([](pplx::task<std::shared_ptr<ModerationAPICountCommentsResponse>> task){
    try{
        auto resp = task.get();
        auto copy = std::make_shared<ModerationAPICountCommentsResponse>(*resp);
        std::cout << "Count: " << copy->count << std::endl;
    }catch(const std::exception& e){
        std::cerr << "Error: " << e.what() << std::endl;
    }
});
[inline-code-end]
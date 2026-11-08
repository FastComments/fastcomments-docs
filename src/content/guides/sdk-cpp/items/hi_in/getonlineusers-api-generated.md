किसी पेज के वर्तमान में ऑनलाइन दर्शक: वे लोग जिनका websocket सत्र इस समय पेज के लिए सब्सक्राइब है।
anonCount + totalCount लौटाता है (पूरे रूम के सब्सक्राइबर, जिनमें वे अनाम दर्शक भी शामिल हैं जिन्हें हम एन्यूमरेट नहीं करते)।

## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| urlId | string | हाँ |  |
| options | const GetOnlineUsersOptions& | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/PageUsersOnlineResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'getOnlineUsers उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t urlId = U("article-456");
auto options = std::make_shared<GetOnlineUsersOptions>();
options->maxResults = boost::optional<int>(100);
options->includeInactive = boost::optional<bool>(false);
api->getOnlineUsers(tenantId, urlId, *options).then([](pplx::task<std::shared_ptr<PageUsersOnlineResponse>> task){
    try{
        auto response = task.get();
    }catch(const std::exception&){
    }
});
[inline-code-end]
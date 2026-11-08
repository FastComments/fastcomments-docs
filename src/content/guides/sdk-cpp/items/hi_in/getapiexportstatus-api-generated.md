## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| options | const GetApiExportStatusOptions& | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`ModerationExportStatusResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/ModerationExportStatusResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'getApiExportStatus उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto opts = GetApiExportStatusOptions{};
opts.exportId = boost::make_optional<utility::string_t>(U("export-456"));
api->getApiExportStatus(U("my-tenant-123"), opts)
    .then([](pplx::task<std::shared_ptr<ModerationExportStatusResponse>> t){
        try{
            auto status = t.get();
        }catch(const std::exception&){
        }
    });
[inline-code-end]
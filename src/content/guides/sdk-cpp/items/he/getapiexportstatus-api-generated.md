## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| options | const GetApiExportStatusOptions& | כן |  |

## תגובה

מחזיר: [`ModerationExportStatusResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationExportStatusResponse.h)

## דוגמה

[inline-code-attrs-start title = 'getApiExportStatus דוגמה'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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
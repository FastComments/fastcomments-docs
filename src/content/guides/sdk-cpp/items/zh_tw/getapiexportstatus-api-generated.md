## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| options | const GetApiExportStatusOptions& | Yes |  |

## 回應

回傳: [`ModerationExportStatusResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationExportStatusResponse.h)

## 範例

[inline-code-attrs-start title = 'getApiExportStatus 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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
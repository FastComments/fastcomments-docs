## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| options | const GetApiExportStatusOptions& | はい |  |

## レスポンス

返却: [`ModerationExportStatusResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationExportStatusResponse.h)

## 例

[inline-code-attrs-start title = 'getApiExportStatus の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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
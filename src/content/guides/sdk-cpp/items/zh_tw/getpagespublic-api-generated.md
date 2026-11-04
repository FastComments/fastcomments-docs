列出租戶的頁面。由 FChat 桌面用戶端使用，以填入其房間清單。
每個頁面在解析後的自訂設定中都需要 `enableFChat` 為 true。
需要 SSO 的頁面會根據請求使用者的群組存取權限進行篩選。

## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| options | const GetPagesPublicOptions& | 是 |  |

## 回應

回傳：[`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetPublicPagesResponse.h)

## 範例

[inline-code-attrs-start title = 'getPagesPublic 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
GetPagesPublicOptions options;
options.limit = boost::optional<int>(50);
options.cursor = boost::optional<utility::string_t>(U("cursor-token"));
api->getPagesPublic(tenantId, options).then([](pplx::task<std::shared_ptr<GetPublicPagesResponse>> task){
    try{
        auto response = task.get();
        // process response if needed
    }catch(const std::exception&){
        // handle error if needed
    }
});
[inline-code-end]
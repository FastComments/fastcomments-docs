---
列出租戶的頁面。供 FChat 桌面客戶端用來填充其房間列表。
需要在每個頁面解析後的自訂設定中 `enableFChat` 為 true。
需要 SSO 的頁面會根據請求者的群組存取權限進行過濾。

## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| cursor | string | 否 |  |
| limit | int32_t | 否 |  |
| q | string | 否 |  |
| sortBy | PagesSortBy | 否 |  |
| hasComments | bool | 否 |  |

## 回應

回傳：[`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetPublicPagesResponse.h)

## 範例

[inline-code-attrs-start title = 'getPagesPublic 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
boost::optional<utility::string_t> cursor = utility::string_t(U("cursor_abc"));
boost::optional<int32_t> limit = 50;
boost::optional<utility::string_t> q = utility::string_t(U("status:published"));
boost::optional<PagesSortBy> sortBy = PagesSortBy::NEWEST;
boost::optional<bool> hasComments = true;
api->getPagesPublic(tenantId, cursor, limit, q, sortBy, hasComments)
.then([](std::shared_ptr<GetPublicPagesResponse> resp){
    if (!resp) resp = std::make_shared<GetPublicPagesResponse>();
})
.wait();
[inline-code-end]

---
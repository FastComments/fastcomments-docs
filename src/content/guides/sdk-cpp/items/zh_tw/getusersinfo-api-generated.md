取得租戶的批次使用者資訊。給定 userIds，從 User / SSOUser 回傳顯示資訊。
由留言小工具使用，以豐富剛透過 presence 事件出現的使用者。
無頁面上下文：隱私一律強制執行（私人個人資料會被遮蔽）。

## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| ids | string | 是 |  |

## 回應

回傳：[`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/PageUsersInfoResponse.h)

## 範例

[inline-code-attrs-start title = 'getUsersInfo 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t ids = U("alice@example.com,bob@example.com");
boost::optional<utility::string_t> locale = boost::make_optional(U("en-US"));

api->getUsersInfo(tenantId, ids).then([](pplx::task<std::shared_ptr<PageUsersInfoResponse>> t){
    try{
        auto response = t.get();
        // 處理回應
    }catch(const std::exception&){
        // 處理錯誤
    }
});
[inline-code-end]
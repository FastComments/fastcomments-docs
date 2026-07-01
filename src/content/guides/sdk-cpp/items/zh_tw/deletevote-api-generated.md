## 參數

| 名稱 | 類型 | 必需 | 說明 |
|------|------|------|------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| editKey | string | 否 |  |

## 回應

返回：[`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/VoteDeleteResponse.h)

## 範例

[inline-code-attrs-start title = 'deleteVote 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto voteId = utility::conversions::to_string_t("vote-9876");
boost::optional<utility::string_t> editKey = utility::conversions::to_string_t("edit-abc123");

api->deleteVote(tenantId, voteId, editKey).then([](pplx::task<std::shared_ptr<VoteDeleteResponse>> task) {
    try {
        auto response = task.get();
        // 按需求處理回應
    } catch (const std::exception&) {
        // 處理錯誤
    }
});
[inline-code-end]
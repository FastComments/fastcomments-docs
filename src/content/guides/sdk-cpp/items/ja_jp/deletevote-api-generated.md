## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |
| editKey | string | いいえ |  |

## レスポンス

戻り値: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/VoteDeleteResponse.h)

## 例

[inline-code-attrs-start title = 'deleteVote の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t id = U("vote-987654321");
boost::optional<utility::string_t> editKey = boost::optional<utility::string_t>(U("edit-key-abc123"));

api->deleteVote(tenantId, id, editKey).then([](pplx::task<std::shared_ptr<VoteDeleteResponse>> t){
    try {
        auto resp = t.get();
        auto finalResp = resp ? resp : std::make_shared<VoteDeleteResponse>();
        (void)finalResp;
    } catch (...) {
        auto errorResp = std::make_shared<VoteDeleteResponse>();
        (void)errorResp;
    }
});
[inline-code-end]
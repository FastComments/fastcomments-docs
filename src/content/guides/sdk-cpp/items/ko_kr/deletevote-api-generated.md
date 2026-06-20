## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| editKey | string | 아니요 |  |

## 응답

반환: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/VoteDeleteResponse.h)

## 예제

[inline-code-attrs-start title = 'deleteVote 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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

---
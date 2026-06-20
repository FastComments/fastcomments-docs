## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| fromName | string | 是 |  |

## 回應

回傳: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 範例

[inline-code-attrs-start title = 'sendInvite 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t id = U("user@example.com");
utility::string_t fromName = U("Acme Support");
boost::optional<utility::string_t> note = boost::optional<utility::string_t>(U("Invitation to join comments"));
api->sendInvite(tenantId, id, fromName)
    .then([](pplx::task<std::shared_ptr<APIEmptyResponse>> t){
        try {
            auto resp = t.get();
            auto finalResp = resp ? resp : std::make_shared<APIEmptyResponse>();
            (void)finalResp;
        } catch (const std::exception&) {
        }
    });
[inline-code-end]

---
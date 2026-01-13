## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| broadcastId | string | 是 |  |
| sso | string | 否 |  |

## 回應

回傳: [`LockComment_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/LockComment_200_response.h)

## 範例

[inline-code-attrs-start title = 'lockComment 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("cmt-456");
utility::string_t broadcastId = U("brdcst-789");
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));

api->lockComment(tenantId, commentId, broadcastId, sso)
    .then([](pplx::task<std::shared_ptr<LockComment_200_response>> t)
    {
        try
        {
            auto resp = t.get();
            if(!resp)
            {
                resp = std::make_shared<LockComment_200_response>();
            }
        }
        catch(const std::exception& ex)
        {
            (void)ex;
        }
    });
[inline-code-end]
## Παράμετροι

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| commentId | string | Ναι |  |
| broadcastId | string | Ναι |  |
| sso | string | Όχι |  |

## Απόκριση

Επιστρέφει: [`LockComment_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/LockComment_200_response.h)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα lockComment'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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

---
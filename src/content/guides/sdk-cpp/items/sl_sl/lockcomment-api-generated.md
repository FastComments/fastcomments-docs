## Parametri

| Ime | Tip | Zahtevano | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| commentId | string | Da |  |
| broadcastId | string | Da |  |
| sso | string | Ne |  |

## Odgovor

Vrne: [`LockComment_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/LockComment_200_response.h)

## Primer

[inline-code-attrs-start title = 'Primer lockComment'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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
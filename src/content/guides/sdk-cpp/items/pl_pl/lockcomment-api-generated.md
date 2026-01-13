## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| commentId | string | Tak |  |
| broadcastId | string | Tak |  |
| sso | string | Nie |  |

## Odpowiedź

Zwraca: [`LockComment_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/LockComment_200_response.h)

## Przykład

[inline-code-attrs-start title = 'Przykład lockComment'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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
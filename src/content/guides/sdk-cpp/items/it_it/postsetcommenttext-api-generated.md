## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| commentId | string | Sì |  |
| setCommentTextParams | SetCommentTextParams | Sì |  |
| options | const PostSetCommentTextOptions& | Sì |  |

## Risposta

Restituisce: [`SetCommentTextResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/SetCommentTextResponse.h)

## Esempio

[inline-code-attrs-start title = 'Esempio postSetCommentText'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = U("my-tenant-123");
auto commentId = U("comment-456");
SetCommentTextParams params;
params.text = U("Revised comment content");
PostSetCommentTextOptions options;
options.requestId = boost::optional<utility::string_t>(U("req-987"));
api->postSetCommentText(tenantId, commentId, params, options)
    .then([](std::shared_ptr<SetCommentTextResponse> resp) {
        auto updatedId = resp->commentId;
    })
    .then([](pplx::task<void> t) {
        try { t.get(); } catch (const std::exception&) {}
    });
[inline-code-end]

---
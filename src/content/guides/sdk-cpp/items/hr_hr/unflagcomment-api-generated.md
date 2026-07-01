## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Da |  |
| options | const UnFlagCommentOptions& | Da |  |

## Odgovor

Vraća: [`FlagCommentResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentResponse.h)

## Primjer

[inline-code-attrs-start title = 'unFlagComment primjer'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto options = UnFlagCommentOptions{};
options.reason = boost::optional<utility::string_t>(U("Resolved by moderator"));
api->unFlagComment(U("my-tenant-123"), U("comment-456"), options)
    .then([](std::shared_ptr<FlagCommentResponse> response) {
        if (response) {
            auto status = response->status;
            // obradi status po potrebi
        }
    })
    .then([](pplx::task<void> previous) {
        try {
            previous.get();
        } catch (const std::exception& e) {
            // obradi grešku
        }
    });
[inline-code-end]
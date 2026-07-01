## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Tak |  |
| postId | string | Tak |  |
| reactBodyParams | ReactBodyParams | Tak |  |
| options | const ReactFeedPostPublicOptions& | Tak |  |

## Odpowiedź

Zwraca: [`ReactFeedPostResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ReactFeedPostResponse.h)

## Przykład

[inline-code-attrs-start title = 'Przykład reactFeedPostPublic'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto postId = utility::conversions::to_string_t("post-987");
ReactBodyParams reactBody;
reactBody.reaction = utility::conversions::to_string_t("love");
reactBody.userId = utility::conversions::to_string_t("user@example.com");
ReactFeedPostPublicOptions options;
options.metadata = boost::optional<utility::string_t>(utility::conversions::to_string_t("mobile"));
api->reactFeedPostPublic(tenantId, postId, reactBody, options)
    .then([](std::shared_ptr<ReactFeedPostResponse> resp) {
    });
[inline-code-end]
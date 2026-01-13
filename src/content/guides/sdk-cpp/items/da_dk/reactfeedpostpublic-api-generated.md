## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| postId | string | Ja |  |
| reactBodyParams | ReactBodyParams | Ja |  |
| isUndo | bool | Nej |  |
| broadcastId | string | Nej |  |
| sso | string | Nej |  |

## Svar

Returnerer: [`ReactFeedPostPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ReactFeedPostPublic_200_response.h)

## Eksempel

[inline-code-attrs-start title = 'reactFeedPostPublic Eksempel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t postId = U("post-987");
ReactBodyParams reactBodyParams;
reactBodyParams.userId = U("jane.doe@example.com");
reactBodyParams.reaction = U("like");
boost::optional<bool> isUndo = boost::optional<bool>(false);
boost::optional<utility::string_t> broadcastId = boost::optional<utility::string_t>(U("broadcast-001"));
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("sso-token-xyz"));
api->reactFeedPostPublic(tenantId, postId, reactBodyParams, isUndo, broadcastId, sso)
    .then([](pplx::task<std::shared_ptr<ReactFeedPostPublic_200_response>> task) {
        try {
            auto resp = task.get();
            auto out = resp ? resp : std::make_shared<ReactFeedPostPublic_200_response>();
            std::cout << "Reaction processed\n";
        } catch (const std::exception &e) {
            std::cerr << "React failed: " << e.what() << '\n';
        }
    });
[inline-code-end]
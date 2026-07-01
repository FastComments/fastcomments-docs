## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| commentId | string | Oui |  |
| direction | string | Oui |  |
| options | const CreateVoteOptions& | Oui |  |

## Réponse

Renvoie : [`VoteResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/VoteResponse.h)

## Exemple

[inline-code-attrs-start title = 'Exemple createVote'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto commentId = utility::conversions::to_string_t("cmt-456");
auto direction = utility::conversions::to_string_t("up");
auto optionsPtr = std::make_shared<CreateVoteOptions>();
optionsPtr->userId = utility::conversions::to_string_t("user-789");
optionsPtr->ipAddress = boost::optional<utility::string_t>(utility::conversions::to_string_t("192.168.1.100"));
api->createVote(tenantId, commentId, direction, *optionsPtr)
    .then([](std::shared_ptr<VoteResponse> resp){});
[inline-code-end]
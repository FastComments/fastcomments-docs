## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| id | string | Oui |  |
| updatableCommentParams | UpdatableCommentParams | Oui |  |
| options | const UpdateCommentOptions& | Oui |  |

## Réponse

Renvoie : [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Exemple

[inline-code-attrs-start title = 'Exemple de updateComment'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t("my-tenant-123");
auto commentId = utility::string_t("cmt-789");
UpdatableCommentParams updatable;
updatable.body = utility::string_t("Edited comment content");
updatable.isSpam = boost::optional<bool>(false);
UpdateCommentOptions options;
options.notify = boost::optional<bool>(true);
api->updateComment(tenantId, commentId, updatable, options)
    .then([](std::shared_ptr<APIEmptyResponse>) {})
    .then([](pplx::task<void> t) { try { t.get(); } catch(const std::exception&) {} });
[inline-code-end]
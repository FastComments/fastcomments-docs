## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| id | string | Ναι |  |
| updateEmailTemplateBody | UpdateEmailTemplateBody | Ναι |  |

## Απόκριση

Επιστρέφει: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα updateEmailTemplate'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = utility::conversions::to_string_t("my-tenant-123");
utility::string_t templateId = utility::conversions::to_string_t("tmpl_welcome_001");
auto bodyPtr = std::make_shared<UpdateEmailTemplateBody>();
bodyPtr->subject = boost::optional<utility::string_t>(utility::conversions::to_string_t("Welcome to ExampleApp"));
bodyPtr->htmlBody = utility::conversions::to_string_t("<p>Hi \{{user.name}}, welcome to ExampleApp!</p>");
bodyPtr->enabled = boost::optional<bool>(true);
api->updateEmailTemplate(tenantId, templateId, *bodyPtr)
.then([](pplx::task<std::shared_ptr<APIEmptyResponse>> task){
  try {
    auto resp = task.get();
    (void)resp;
  } catch (const std::exception &e) {
  }
});
[inline-code-end]

---
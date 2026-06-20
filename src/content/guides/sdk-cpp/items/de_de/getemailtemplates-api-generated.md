## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| skip | double | Nein |  |

## Antwort

Gibt zurück: [`GetEmailTemplatesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetEmailTemplatesResponse.h)

## Beispiel

[inline-code-attrs-start title = 'getEmailTemplates Beispiel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<double> skip = boost::optional<double>(0.0);
api->getEmailTemplates(tenantId, skip)
.then([](std::shared_ptr<GetEmailTemplatesResponse> resp) -> std::shared_ptr<GetEmailTemplatesResponse> {
    auto finalResp = resp ? resp : std::make_shared<GetEmailTemplatesResponse>();
    return finalResp;
})
.wait();
[inline-code-end]

---
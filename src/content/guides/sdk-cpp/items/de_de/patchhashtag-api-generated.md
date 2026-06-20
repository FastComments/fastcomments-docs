---
## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tag | string | Ja |  |
| tenantId | string | Nein |  |
| updateHashTagBody | UpdateHashTagBody | Nein |  |

## Antwort

Gibt zurück: [`UpdateHashTagResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UpdateHashTagResponse.h)

## Beispiel

[inline-code-attrs-start title = 'patchHashTag Beispiel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tag = U("bug");
boost::optional<utility::string_t> tenantId{ U("my-tenant-123") };
UpdateHashTagBody body;
boost::optional<UpdateHashTagBody> updateBody{ body };
api->patchHashTag(tag, tenantId, updateBody)
.then([](std::shared_ptr<UpdateHashTagResponse> resp)
{
    if (resp)
    {
        auto localCopy = std::make_shared<UpdateHashTagResponse>(*resp);
    }
});
[inline-code-end]

---
## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tag | string | Da |  |
| tenantId | string | Ne |  |
| deleteHashTagRequest | DeleteHashTag_request | Ne |  |

## Odgovor

Vraća: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## Primer

[inline-code-attrs-start title = 'deleteHashTag Primer'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tag = U("inappropriate");
boost::optional<utility::string_t> tenantId = boost::optional<utility::string_t>(U("my-tenant-123"));
DeleteHashTag_request req;
boost::optional<DeleteHashTag_request> optReq = boost::optional<DeleteHashTag_request>(req);

api->deleteHashTag(tag, tenantId, optReq)
.then([](std::shared_ptr<FlagCommentPublic_200_response> resp){
    auto out = std::make_shared<FlagCommentPublic_200_response>(*resp);
    std::cout << "Hashtag removed successfully for tenant\n";
    return out;
}).wait();
[inline-code-end]

---
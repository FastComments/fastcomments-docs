---
## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Nein |  |
| createHashTagBody | CreateHashTagBody | Nein |  |

## Antwort

Rückgabe: [`AddHashTag_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/AddHashTag_200_response.h)

## Beispiel

[inline-code-attrs-start title = 'addHashTag Beispiel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> tenantId(U("my-tenant-123"));
auto bodyPtr = std::make_shared<CreateHashTagBody>();
bodyPtr->name = U("release");
bodyPtr->color = U("#00aaff");
boost::optional<CreateHashTagBody> createBody(*bodyPtr);
api->addHashTag(tenantId, createBody).then([](pplx::task<std::shared_ptr<AddHashTag_200_response>> t){
    try {
        auto resp = t.get();
        (void)resp;
    } catch(...) {}
});
[inline-code-end]

---
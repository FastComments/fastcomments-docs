## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| id | string | Oui |  |

## Réponse

Retourne : [`GetQuestionConfigResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetQuestionConfigResponse.h)

## Exemple

[inline-code-attrs-start title = 'Exemple getQuestionConfig'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t questionId = U("qstn-456");
boost::optional<utility::string_t> ifNoneMatch = U("W/\"5a2f3c\"");
api->getQuestionConfig(tenantId, questionId)
.then([ifNoneMatch](pplx::task<std::shared_ptr<GetQuestionConfigResponse>> t){
    try {
        auto resp = t.get();
        if (ifNoneMatch) {
            auto etag = *ifNoneMatch;
            (void)etag;
        }
        auto cfg = resp ? resp : std::make_shared<GetQuestionConfigResponse>();
        (void)cfg;
    } catch (const std::exception &ex) {
        (void)ex;
    }
});
[inline-code-end]

---
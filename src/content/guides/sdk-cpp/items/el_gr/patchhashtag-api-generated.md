## Παράμετροι

| Name | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tag | string | Ναι |  |
| tenantId | string | Όχι |  |
| updateHashTagBody | UpdateHashTagBody | Όχι |  |

## Απάντηση

Επιστρέφει: [`UpdateHashTagResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UpdateHashTagResponse.h)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα patchHashTag'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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
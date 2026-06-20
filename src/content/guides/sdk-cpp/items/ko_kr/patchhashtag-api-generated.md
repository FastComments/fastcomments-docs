## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tag | string | 예 |  |
| tenantId | string | 아니오 |  |
| updateHashTagBody | UpdateHashTagBody | 아니오 |  |

## 응답

반환: [`UpdateHashTagResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UpdateHashTagResponse.h)

## 예제

[inline-code-attrs-start title = 'patchHashTag 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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
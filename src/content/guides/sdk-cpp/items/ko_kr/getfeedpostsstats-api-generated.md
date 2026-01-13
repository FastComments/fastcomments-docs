## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| postIds | vector<string | 예 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`GetFeedPostsStats_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetFeedPostsStats_200_response.h)

## 예제

[inline-code-attrs-start title = 'getFeedPostsStats 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
std::vector<utility::string_t> postIds{ U("post-abc-001"), U("post-xyz-123") };
boost::optional<utility::string_t> sso(U("user@example.com"));
api->getFeedPostsStats(tenantId, postIds, sso)
    .then([](pplx::task<std::shared_ptr<GetFeedPostsStats_200_response>> t)
    {
        try {
            auto resp = t.get();
            auto respCopy = resp ? std::make_shared<GetFeedPostsStats_200_response>(*resp) : nullptr;
            (void)respCopy;
        } catch (const std::exception&) {}
    });
[inline-code-end]

---
## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |

## 응답

반환: [`APIGetUserBadgeProgressResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIGetUserBadgeProgressResponse.h)

## 예시

[inline-code-attrs-start title = 'getUserBadgeProgressById 예시'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = U("my-tenant-123");
auto userId = U("user-456");
api->getUserBadgeProgressById(tenantId, userId)
    .then([=](pplx::task<std::shared_ptr<APIGetUserBadgeProgressResponse>> t){
        try{
            auto resp = t.get();
            boost::optional<std::shared_ptr<APIGetUserBadgeProgressResponse>> optResp = resp;
            if(optResp){}
        }catch(const std::exception&){}
    });
[inline-code-end]
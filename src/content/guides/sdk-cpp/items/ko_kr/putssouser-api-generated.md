---
## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| updateAPISSOUserData | UpdateAPISSOUserData | 예 |  |
| updateComments | bool | 아니오 |  |

## 응답

반환: [`PutSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PutSSOUserAPIResponse.h)

## 예제

[inline-code-attrs-start title = 'putSSOUser 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t id = U("user@example.com");
UpdateAPISSOUserData updateData;
updateData.displayName = U("Jane Doe");
updateData.email = U("user@example.com");
boost::optional<bool> updateComments = true;
api->putSSOUser(tenantId, id, updateData, updateComments)
.then([](std::shared_ptr<PutSSOUserAPIResponse> resp){
    if(!resp){ std::cout << "putSSOUser returned null\n"; return; }
    auto copy = std::make_shared<PutSSOUserAPIResponse>(*resp);
    std::cout << "SSO user updated successfully\n";
});
[inline-code-end]

---
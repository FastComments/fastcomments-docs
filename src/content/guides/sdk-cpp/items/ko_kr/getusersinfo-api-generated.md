---
테넌트의 대량 사용자 정보. userIds를 받아 User / SSOUser의 표시 정보를 반환합니다.
댓글 위젯에서 presence 이벤트로 방금 등장한 사용자를 보강하는 데 사용됩니다.
페이지 컨텍스트 없음: 개인정보는 일관되게 적용됩니다(비공개 프로필은 마스킹 처리됨).

## 매개변수

| 이름 | Type | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| ids | string | 예 |  |

## 응답

반환: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PageUsersInfoResponse.h)

## 예제

[inline-code-attrs-start title = 'getUsersInfo 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t ids = U("alice@example.com,bob@example.com");
boost::optional<utility::string_t> statusFilter = U("active");
api->getUsersInfo(tenantId, ids).then([](pplx::task<std::shared_ptr<PageUsersInfoResponse>> t){
    try {
        auto res = t.get();
        if (res) {
            auto responseCopy = std::make_shared<PageUsersInfoResponse>(*res);
        }
    } catch (const std::exception&) {}
});
[inline-code-end]

---
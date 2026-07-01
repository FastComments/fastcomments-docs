테넌트에 대한 대량 사용자 정보. 주어진 userIds에 대해 User / SSOUser의 표시 정보를 반환합니다.  
댓글 위젯에서 존재 이벤트를 통해 방금 등장한 사용자를 풍부하게 표시하기 위해 사용됩니다.  
페이지 컨텍스트가 없으며: 프라이버시가 일관되게 적용됩니다 (비공개 프로필은 마스킹됩니다).

## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| ids | string | Yes |  |

## 응답

반환: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PageUsersInfoResponse.h)

## 예시

[inline-code-attrs-start title = 'getUsersInfo 예시'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t ids = U("alice@example.com,bob@example.com");
boost::optional<utility::string_t> locale = boost::make_optional(U("en-US"));

api->getUsersInfo(tenantId, ids).then([](pplx::task<std::shared_ptr<PageUsersInfoResponse>> t){
    try{
        auto response = t.get();
        // 응답 처리
    }catch(const std::exception&){
        // 오류 처리
    }
});
[inline-code-end]
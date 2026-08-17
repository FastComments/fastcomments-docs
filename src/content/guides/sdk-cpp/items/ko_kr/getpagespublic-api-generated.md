테넌트의 페이지 목록을 가져옵니다. FChat 데스크톱 클라이언트가 방 목록을 채우는 데 사용됩니다. 각 페이지에 대한 해결된 사용자 정의 구성에서 `enableFChat`가 true이어야 합니다. SSO가 필요한 페이지는 요청 사용자의 그룹 접근 권한에 따라 필터링됩니다.

## Parameters

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| options | const GetPagesPublicOptions& | Yes |  |

## 응답

반환: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetPublicPagesResponse.h)

## 예제

[inline-code-attrs-start title = 'getPagesPublic 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
GetPagesPublicOptions options;
options.limit = boost::optional<int>(50);
options.cursor = boost::optional<utility::string_t>(U("cursor-token"));
api->getPagesPublic(tenantId, options).then([](pplx::task<std::shared_ptr<GetPublicPagesResponse>> task){
    try{
        auto response = task.get();
        // 필요에 따라 응답 처리
    }catch(const std::exception&){
        // 필요에 따라 오류 처리
    }
});
[inline-code-end]

---
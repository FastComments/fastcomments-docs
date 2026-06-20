---
테넌트에 대한 대량 사용자 정보입니다. userIds가 주어지면 User / SSOUser로부터 표시 정보를 반환합니다.
댓글 위젯에서 presence 이벤트로 막 나타난 사용자들의 정보를 보강하는 데 사용됩니다.
페이지 컨텍스트 없음: 개인 정보 보호가 일관되게 적용됩니다(비공개 프로필은 마스킹됨).

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| ids | String | 예 |  |

## Response

반환: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_info_response.rs)

## 예제

[inline-code-attrs-start title = 'get_users_info 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: GetUsersInfoParams = GetUsersInfoParams {
    tenant_id: "acme-corp-tenant".to_string(),
    ids: "alice@example.com,bob@example.com,carol@example.com".to_string(),
    page_size: Some(100),
};
let users_response: PageUsersInfoResponse = get_users_info(&configuration, params).await?;
[inline-code-end]

---
---
테넌트에 대한 대량 사용자 정보를 제공합니다. userIds가 주어지면 User / SSOUser의 표시 정보를 반환합니다. 댓글 위젯에서 존재 이벤트를 통해 새로 나타난 사용자를 풍부하게 표시하기 위해 사용됩니다. 페이지 컨텍스트가 없으며, 프라이버시는 일관되게 적용됩니다 (비공개 프로필은 마스킹됩니다).

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| ids | String | Yes |  |

## Response

반환: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_info_response.rs)

## Example

[inline-code-attrs-start title = 'get_users_info 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params = GetUsersInfoParams {
    tenant_id: "acme-corp-tenant".to_string(),
    ids: "user-1,user-2".to_string(),
};
let page: PageUsersInfoResponse = get_users_info(&configuration, params).await?;
[inline-code-end]

---
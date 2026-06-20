---
## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|------|-------------|
| tenant_id | String | 예 |  |
| id | String | 예 |  |

## 응답

반환: [`ApiEmptySuccessResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_success_response.rs)

## 예제

[inline-code-attrs-start title = 'delete_user_badge 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: DeleteUserBadgeParams = DeleteUserBadgeParams {
    tenant_id: "acme-newsroom-tenant".to_string(),
    id: "badge-moderator-001".to_string(),
};
let include_related: Option<bool> = Some(false);
let result: ApiEmptySuccessResponse = delete_user_badge(&configuration, params).await?;
[inline-code-end]

---
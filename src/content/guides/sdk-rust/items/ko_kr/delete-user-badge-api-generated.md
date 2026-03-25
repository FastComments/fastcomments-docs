## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| id | String | 예 |  |

## 응답

반환: [`UpdateUserBadge200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_badge_200_response.rs)

## 예제

[inline-code-attrs-start title = 'delete_user_badge 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_delete_badge() -> Result<UpdateUserBadge200Response, Error> {
    let params: DeleteUserBadgeParams = DeleteUserBadgeParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "badge-7890".to_string(),
    };
    let response: UpdateUserBadge200Response = delete_user_badge(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---
## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| skip | i32 | No |  |

## 응답

반환: [`GetSsoUsersResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_sso_users_response.rs)

## 예시

[inline-code-attrs-start title = 'get_sso_users 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetSsoUsersParams {
        tenant_id: "acme-corp-tenant".to_string(),
        skip: Some(10),
    };
    let _response = get_sso_users(&configuration, params).await?;
    Ok(())
}
[inline-code-end]
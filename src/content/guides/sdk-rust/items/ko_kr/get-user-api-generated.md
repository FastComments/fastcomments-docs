---
## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| id | String | 예 |  |

## 응답

반환: [`GetUserResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_response.rs)

## 예제

[inline-code-attrs-start title = 'get_user 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_get_user() -> Result<(), Error> {
    let params = GetUserParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "user-7b9a2c".to_string(),
        include_roles: Some(true),
    };
    let user: GetUserResponse = get_user(&configuration, params).await?;
    println!("{:#?}", user);
    Ok(())
}
[inline-code-end]

---
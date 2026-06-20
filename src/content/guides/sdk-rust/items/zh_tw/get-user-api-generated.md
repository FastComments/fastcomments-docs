## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| id | String | 是 |  |

## 回應

回傳： [`GetUserResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_response.rs)

## 範例

[inline-code-attrs-start title = 'get_user 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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
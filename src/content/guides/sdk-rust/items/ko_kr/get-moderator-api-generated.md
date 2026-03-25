## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| id | String | 예 |  |

## 응답

반환: [`GetModerator200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_moderator_200_response.rs)

## 예제

[inline-code-attrs-start title = 'get_moderator 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params = GetModeratorParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "moderator-4521".to_string(),
        include_permissions: Some(true),
    };
    let moderator: GetModerator200Response = get_moderator(&configuration, params).await?;
    println!("{:#?}", moderator);
    Ok(())
}
[inline-code-end]

---
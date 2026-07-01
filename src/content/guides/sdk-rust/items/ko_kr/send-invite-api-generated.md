## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |
| from_name | String | Yes |  |

## 응답

반환: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## 예시

[inline-code-attrs-start title = 'send_invite 예시'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = SendInviteParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/article".to_string(),
        from_name: "John Doe".to_string(),
        message: Some("Welcome to the platform".to_string()),
        ..Default::default()
    };
    let _ = send_invite(configuration, params).await?;
    Ok(())
}
[inline-code-end]

---
## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| id | String | 예 |  |
| redirect_url | String | 아니오 |  |

## 응답

반환: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## 예제

[inline-code-attrs-start title = 'send_login_link 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn send_link_example() -> Result<(), Error> {
    let params: SendLoginLinkParams = SendLoginLinkParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "user-9876".to_string(),
        redirect_url: Some("https://acme.example.com/welcome".to_string()),
    };
    let response: ApiEmptyResponse = send_login_link(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---
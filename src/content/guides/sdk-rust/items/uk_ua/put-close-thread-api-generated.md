## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| url_id | String | Так |  |
| sso | String | Ні |  |

## Відповідь

Повертає: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Приклад

[inline-code-attrs-start title = 'put_close_thread приклад'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn close_thread() -> Result<(), Error> {
    let params: PutCloseThreadParams = PutCloseThreadParams {
        url_id: String::from("news/2026/07/acme-launch-coverage"),
        sso: Some(String::from("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.sso_payload.signature")),
    };
    let response: ApiEmptyResponse = put_close_thread(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---
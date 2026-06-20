## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| skip | f64 | 아니요 |  |

## 응답

반환: [`GetEmailTemplatesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_email_templates_response.rs)

## 예제

[inline-code-attrs-start title = 'get_email_templates 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_templates() -> Result<GetEmailTemplatesResponse, Error> {
    let params: GetEmailTemplatesParams = GetEmailTemplatesParams {
        tenant_id: "acme-corp-tenant".to_string(),
        skip: Some(10.0),
    };
    let response: GetEmailTemplatesResponse = get_email_templates(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---
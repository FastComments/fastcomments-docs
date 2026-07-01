## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|------|------|
| tenant_id | String | 예 |  |
| id | String | 예 |  |

## 응답

반환: [`GetTenantResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenant_response.rs)

## 예시

[inline-code-attrs-start title = 'get_tenant 예시'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetTenantParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/article".to_string(),
        include_billing: Some(true),
    };
    let _response: GetTenantResponse = get_tenant(&configuration, params).await?;
    Ok(())
}
[inline-code-end]
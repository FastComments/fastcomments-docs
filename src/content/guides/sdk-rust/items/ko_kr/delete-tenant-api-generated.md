## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |
| sure | String | No |  |

## 응답

반환: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## 예제

[inline-code-attrs-start title = 'delete_tenant 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: DeleteTenantParams = DeleteTenantParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "tenant-5f2d".to_string(),
        sure: Some("confirm".to_string()),
    };
    let response: ApiEmptyResponse = delete_tenant(&configuration, params).await?;
    let _ = response;
    Ok(())
}
[inline-code-end]

---
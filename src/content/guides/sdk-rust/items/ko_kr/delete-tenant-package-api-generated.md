## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |

## 응답

반환: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## 예제

[inline-code-attrs-start title = 'delete_tenant_package 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_delete() -> Result<FlagCommentPublic200Response, Error> {
    let params: DeleteTenantPackageParams = DeleteTenantPackageParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/article-comments-package-2026-03".to_string(),
    };
    let response: FlagCommentPublic200Response = delete_tenant_package(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---
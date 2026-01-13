## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| id | String | 是 |  |
| sure | String | 否 |  |

## 回應

回傳：[`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## 範例

[inline-code-attrs-start title = 'delete_tenant 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_delete_tenant() -> Result<FlagCommentPublic200Response, Error> {
    let params: DeleteTenantParams = DeleteTenantParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/article-12345".to_string(),
        sure: Some("confirm".to_string()),
    };
    let response: FlagCommentPublic200Response = delete_tenant(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---
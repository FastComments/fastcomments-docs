## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tag | String | 是 |  |
| tenant_id | String | 否 |  |
| update_hash_tag_body | models::UpdateHashTagBody | 否 |  |

## 回應

回傳: [`PatchHashTag200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/patch_hash_tag_200_response.rs)

## 範例

[inline-code-attrs-start title = 'patch_hash_tag 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_patch_hash_tag() -> Result<PatchHashTag200Response, Error> {
    let params: PatchHashTagParams = PatchHashTagParams {
        tag: "breaking-news".to_string(),
        tenant_id: Some("acme-corp-tenant".to_string()),
        update_hash_tag_body: Some(models::UpdateHashTagBody {
            name: "Breaking News".to_string(),
            description: "Posts about breaking news and urgent updates".to_string(),
            synonyms: vec!["breaking".to_string(), "urgent".to_string()],
            is_active: true,
        }),
    };
    let response: PatchHashTag200Response = patch_hash_tag(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---
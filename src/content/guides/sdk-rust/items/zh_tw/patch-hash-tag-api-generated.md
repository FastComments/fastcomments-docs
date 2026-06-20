## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tag | String | 是 |  |
| tenant_id | String | 否 |  |
| update_hash_tag_body | models::UpdateHashTagBody | 否 |  |

## 回應

回傳: [`UpdateHashTagResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_hash_tag_response.rs)

## 範例

[inline-code-attrs-start title = 'patch_hash_tag 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let cfg: &configuration::Configuration = &configuration;
let body: models::UpdateHashTagBody = Default::default();
let params: PatchHashTagParams = PatchHashTagParams {
    tag: "news/article".to_string(),
    tenant_id: Some("acme-corp-tenant".to_string()),
    update_hash_tag_body: Some(body),
};
let response: UpdateHashTagResponse = patch_hash_tag(cfg, params).await?;
[inline-code-end]

---
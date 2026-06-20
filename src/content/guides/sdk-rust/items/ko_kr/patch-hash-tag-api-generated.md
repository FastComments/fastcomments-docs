---
## Parameters

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tag | String | 예 |  |
| tenant_id | String | 아니오 |  |
| update_hash_tag_body | models::UpdateHashTagBody | 아니오 |  |

## 응답

반환: [`UpdateHashTagResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_hash_tag_response.rs)

## 예제

[inline-code-attrs-start title = 'patch_hash_tag 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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
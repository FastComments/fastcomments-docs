## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tag | String | Oui |  |
| tenant_id | String | Non |  |
| update_hash_tag_body | models::UpdateHashTagBody | Non |  |

## Réponse

Renvoie : [`UpdateHashTagResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_hash_tag_response.rs)

## Exemple

[inline-code-attrs-start title = 'patch_hash_tag Exemple'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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
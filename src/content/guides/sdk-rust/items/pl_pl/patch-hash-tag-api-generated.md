## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tag | String | Tak |  |
| tenant_id | String | Nie |  |
| update_hash_tag_body | models::UpdateHashTagBody | Nie |  |

## Odpowiedź

Zwraca: [`PatchHashTag200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/patch_hash_tag_200_response.rs)

## Przykład

[inline-code-attrs-start title = 'Przykład patch_hash_tag'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: PatchHashTagParams = PatchHashTagParams {
    tag: "news/article".to_string(),
    tenant_id: Some("acme-corp-tenant".to_string()),
    update_hash_tag_body: Some(models::UpdateHashTagBody {
        label: Some("World News".to_string()),
        description: Some("Articles related to world events.".to_string()),
        enabled: Some(true),
    }),
};

let response: PatchHashTag200Response = patch_hash_tag(&configuration, params).await?
[inline-code-end]

---
## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| page | f64 | Ne |  |

## Odgovor

Vrača: [`GetHashTags200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_hash_tags_200_response.rs)

## Primer

[inline-code-attrs-start title = 'Primer get_hash_tags'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_hash_tags() -> Result<GetHashTags200Response, Error> {
    let params: GetHashTagsParams = GetHashTagsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        page: Some(1.0),
    };
    let response: GetHashTags200Response = get_hash_tags(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---
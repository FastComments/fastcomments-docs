## Παράμετροι

| Όνομα | Type | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenant_id | String | Ναι |  |
| id | String | Ναι |  |
| replace_tenant_package_body | models::ReplaceTenantPackageBody | Ναι |  |

## Απόκριση

Επιστρέφει: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα replace_tenant_package'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: ReplaceTenantPackageParams = ReplaceTenantPackageParams {
        tenant_id: String::from("acme-corp-tenant"),
        id: String::from("news/article-package"),
        replace_tenant_package_body: models::ReplaceTenantPackageBody {
            name: Some(String::from("Article Comments Package")),
            plan: Some(String::from("pro")),
            enabled: Some(true),
            features: Some(vec![String::from("moderation"), String::from("reactions")]),
            metadata: Some(std::collections::HashMap::from([
                (String::from("region"), String::from("us-east-1")),
                (String::from("contact"), String::from("ops@acme.example")),
            ])),
        },
    };

    let _response: ApiEmptyResponse = replace_tenant_package(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---
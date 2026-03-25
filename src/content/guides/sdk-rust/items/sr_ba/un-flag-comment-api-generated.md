## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| id | String | Da |  |
| user_id | String | Ne |  |
| anon_user_id | String | Ne |  |

## Odgovor

Vraća: [`FlagComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_200_response.rs)

## Primjer

[inline-code-attrs-start title = 'Primjer un_flag_comment'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_unflag_comment() -> Result<FlagComment200Response, Error> {
    let params = UnFlagCommentParams {
        tenant_id: String::from("acme-corp-tenant"),
        id: String::from("news/article/comment-12345"),
        user_id: Some(String::from("reader-987")),
        anon_user_id: None,
    };
    let response: FlagComment200Response = un_flag_comment(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---
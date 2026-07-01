## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|------------|------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |

## Απάντηση

Επιστρέφει: [`GetQuestionConfigResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_question_config_response.rs)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα get_question_config'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetQuestionConfigParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/article".to_string(),
    };
    let _response = get_question_config(&configuration, params).await?;
    Ok(())
}
[inline-code-end]
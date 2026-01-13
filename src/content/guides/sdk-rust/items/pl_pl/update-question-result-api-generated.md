## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenant_id | String | Tak |  |
| id | String | Tak |  |
| update_question_result_body | models::UpdateQuestionResultBody | Tak |  |

## Odpowiedź

Zwraca: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Przykład

[inline-code-attrs-start title = 'Przykład update_question_result'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let body: models::UpdateQuestionResultBody = models::UpdateQuestionResultBody {
        question_id: "q-12345".to_string(),
        result: true,
        comment: Some("Marked by moderator after review".to_string()),
    };

    let params: UpdateQuestionResultParams = UpdateQuestionResultParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/article/98765".to_string(),
        update_question_result_body: body,
    };

    let response: FlagCommentPublic200Response = update_question_result(&configuration, params).await?;
    println!("{:#?}", response);
    Ok(())
}
[inline-code-end]

---
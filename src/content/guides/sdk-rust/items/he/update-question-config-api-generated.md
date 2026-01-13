## פרמטרים

| שם | סוג | חובה | תיאור |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| id | String | כן |  |
| update_question_config_body | models::UpdateQuestionConfigBody | כן |  |

## תגובה

מחזיר: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-update_question_config'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_update() -> Result<(), Error> {
    let params: UpdateQuestionConfigParams = UpdateQuestionConfigParams {
        tenant_id: String::from("acme-corp-tenant"),
        id: String::from("news/article-2026-01-12"),
        update_question_config_body: models::UpdateQuestionConfigBody {
            question_text: String::from("Did you find this reporting accurate?"),
            required: Some(true),
            rendering_type: Some(models::QuestionRenderingType::Inline),
            options: Some(vec![
                models::QuestionConfigCustomOptionsInner { id: String::from("opt-yes"), label: String::from("Yes"), value: String::from("yes") },
                models::QuestionConfigCustomOptionsInner { id: String::from("opt-no"), label: String::from("No"), value: String::from("no") },
            ]),
            when_save: Some(models::QuestionWhenSave::OnSubmit),
        },
    };
    let response: FlagCommentPublic200Response = update_question_config(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---
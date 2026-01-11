## פרמטרים

| שם | סוג | חובה | תיאור |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| id | String | כן |  |
| body | models::PickApiCommentPeriodUpdatableCommentFields | כן |  |
| context_user_id | String | לא |  |
| do_spam_check | bool | לא |  |
| is_live | bool | לא |  |

## תגובה

מחזיר: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

---
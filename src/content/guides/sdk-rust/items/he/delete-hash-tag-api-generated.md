## פרמטרים

| שם | סוג | חובה | תיאור |
|------|------|----------|-------------|
| tag | String | כן |  |
| tenant_id | String | לא |  |
| delete_hash_tag_request_body | models::DeleteHashTagRequestBody | לא |  |

## תגובה

מחזיר: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-delete_hash_tag'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: DeleteHashTagParams = DeleteHashTagParams {
    tag: "news/article".to_string(),
    tenant_id: Some("acme-corp-tenant".to_string()),
    delete_hash_tag_request_body: Some(DeleteHashTagRequestBody {}),
};
let response: ApiEmptyResponse = delete_hash_tag(&configuration, params).await?;
[inline-code-end]

---
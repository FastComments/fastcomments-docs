## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| create_feed_post_params | models::CreateFeedPostParams | כן |  |
| broadcast_id | String | לא |  |
| is_live | bool | לא |  |
| do_spam_check | bool | לא |  |
| skip_dup_check | bool | לא |  |

## תגובה

מחזיר: [`CreateFeedPost200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_feed_post_200_response.rs)

---
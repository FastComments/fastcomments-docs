## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| create_feed_post_params | models::CreateFeedPostParams | 예 |  |
| broadcast_id | String | 아니오 |  |
| is_live | bool | 아니오 |  |
| do_spam_check | bool | 아니오 |  |
| skip_dup_check | bool | 아니오 |  |

## 응답

반환: [`CreateFeedPost200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_feed_post_200_response.rs)

---
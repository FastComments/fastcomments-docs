---
특정 댓글에 대한 알림을 활성화하거나 비활성화합니다.

## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|------|-------------|
| tenant_id | String | 예 |  |
| notification_id | String | 예 |  |
| opted_in_or_out | String | 예 |  |
| comment_id | String | 예 |  |
| sso | String | 아니요 |  |

## 응답

반환: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_notification_status_200_response.rs)

---
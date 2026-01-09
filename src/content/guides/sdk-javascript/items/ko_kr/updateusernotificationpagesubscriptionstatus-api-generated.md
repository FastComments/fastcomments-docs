페이지에 대한 알림을 활성화하거나 비활성화합니다. 사용자가 페이지를 구독하면 새 최상위 댓글에 대한 알림이 생성되며, 또한

## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| urlId | string | 예 |  |
| url | string | 예 |  |
| pageTitle | string | 예 |  |
| subscribedOrUnsubscribed | UpdateUserNotificationPageSubscriptionStatusSubscribedOrUnsubscribedEnum | 예 |  |
| sso | string | 아니요 |  |

## 응답

반환: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserNotificationStatus200Response.ts)

---
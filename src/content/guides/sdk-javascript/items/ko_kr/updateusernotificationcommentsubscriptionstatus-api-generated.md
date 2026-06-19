특정 댓글에 대한 알림을 활성화하거나 비활성화합니다.

## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| notificationId | string | 예 |  |
| optedInOrOut | UpdateUserNotificationCommentSubscriptionStatusOptedInOrOutEnum | 예 |  |
| commentId | string | 예 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`UpdateUserNotificationCommentSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserNotificationCommentSubscriptionStatusResponse.ts)

## 예제

[inline-code-attrs-start title = 'updateUserNotificationCommentSubscriptionStatus 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_92a1c4';
const notificationId: string = 'notif_3f7b2e9a-4d1b-11ec-8d3d-0242ac130003';
const optedInOrOut: UpdateUserNotificationCommentSubscriptionStatusOptedInOrOutEnum =
  UpdateUserNotificationCommentSubscriptionStatusOptedInOrOutEnum.OPTED_IN;
const commentId: string = 'cmt_8f4b2e7a';
const sso: string | undefined = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.fake.payload';
const response: UpdateUserNotificationCommentSubscriptionStatusResponse =
  await updateUserNotificationCommentSubscriptionStatus(tenantId, notificationId, optedInOrOut, commentId, sso);
[inline-code-end]

---
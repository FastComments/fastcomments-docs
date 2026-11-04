페이지에 대한 알림을 활성화하거나 비활성화합니다. 사용자가 페이지를 구독하면 새로운 최상위 댓글에 대해 알림이 생성되며, 또한

## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| urlId | string | 예 |  |
| url | string | 예 |  |
| pageTitle | string | 예 |  |
| subscribedOrUnsubscribed | UpdateUserNotificationPageSubscriptionStatusSubscribedOrUnsubscribedEnum | 예 |  |
| sso | string | 아니요 |  |

## 응답

반환값: [`UpdateUserNotificationPageSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserNotificationPageSubscriptionStatusResponse.ts)

## 예제

[inline-code-attrs-start title = 'updateUserNotificationPageSubscriptionStatus 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample() {
  const tenantId: string = "tenant_12345";
  const urlId: string = "url_9876";
  const url: string = "https://example.com/articles/awesome-article";
  const pageTitle: string = "Awesome Article";
  const subscribedOrUnsubscribed: UpdateUserNotificationPageSubscriptionStatusSubscribedOrUnsubscribedEnum =
    UpdateUserNotificationPageSubscriptionStatusSubscribedOrUnsubscribedEnum.Subscribed;
  const sso: string = "sso_token_abc";

  const response: UpdateUserNotificationPageSubscriptionStatusResponse = await updateUserNotificationPageSubscriptionStatus(
    tenantId,
    urlId,
    url,
    pageTitle,
    subscribedOrUnsubscribed,
    sso
  );

  console.log(response);
}

runExample();
[inline-code-end]
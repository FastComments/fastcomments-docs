`Subscription` 객체는 사용자의 구독을 나타냅니다.

`Subscription` 객체는 사용자가 댓글 위젯의 알림 벨을 클릭하고 "이 페이지 구독"을 클릭할 때 생성됩니다.

구독은 API를 통해서도 생성할 수 있습니다.

`Subscription` 객체가 있으면 연관된 페이지의 루트에 새 댓글이 남겨질 때
해당 `Subscription`의 대상인 페이지에 대해 `Notification` 객체가 생성되고 이메일이 전송됩니다. 이메일 전송은 사용자 유형에 따라 달라집니다. 일반 사용자에게는 `optedInNotifications`에 따라 결정됩니다. SSO 사용자에게는 `optedInSubscriptionNotifications`에 따라 결정됩니다. 일부 애플리케이션은 웹에서 접근 가능한 페이지 개념이 없을 수 있으므로, 그런 경우 구독하려는 항목의 id(댓글 위젯에 전달할 `urlId`와 동일한 값)를 `urlId`에 설정하면 됩니다.

`Subscription` 객체의 구조는 다음과 같습니다:

[inline-code-attrs-start title = '구독 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Subscription {
    id: string
    tenantId: string
    /** SSO의 경우, 사용자 id는 `<tenant id>:<user id>` 형식입니다. **/
    userId: string
    anonUserId?: string
    urlId: string
    url?: string
    pageTitle?: string
    createdAt: string // 날짜 문자열
}
[inline-code-end]

---
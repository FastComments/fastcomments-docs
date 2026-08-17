---
`NotificationCount` 객체는 사용자의 읽지 않은 알림 수와 메타데이터를 나타냅니다.

읽지 않은 알림이 없으면 해당 사용자에 대한 `NotificationCount`가 존재하지 않습니다.

`NotificationCount` 객체는 자동으로 생성되며 API를 통해 생성할 수 없습니다. 또한 1년 후에 만료됩니다.

사용자의 `NotificationCount`를 삭제하면 읽지 않은 알림 수를 초기화할 수 있습니다.

`NotificationCount` 객체의 구조는 다음과 같습니다:

[inline-code-attrs-start title = 'NotificationCount 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCount {
    id: string // 사용자 ID
    count: number
    createdAt: string // 날짜 문자열
    expireAt: string // 날짜 문자열
}
[inline-code-end]

---
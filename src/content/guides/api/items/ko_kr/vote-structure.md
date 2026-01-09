---
`Vote` 객체는 사용자가 남긴 투표를 나타냅니다.

댓글과 투표 간의 관계는 `commentId`로 정의됩니다.

Vote 객체의 구조는 다음과 같습니다:

[inline-code-attrs-start title = 'Vote 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Vote {
    id: string
    urlId: string
    commentId: string
    userId: string
    direction: 1 | -1
    createdAt: string
}
[inline-code-end]

---
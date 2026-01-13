A `Page` 객체는 여러 댓글이 속할 수 있는 페이지를 나타냅니다. 이 관계는 `urlId`로 정의됩니다.

A `Page`는 페이지 제목, 댓글 수, 및 `urlId`와 같은 정보를 저장합니다.

Page 객체의 구조는 다음과 같습니다:

[inline-code-attrs-start title = '페이지 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Page {
    id: string
    urlId: string
    url: string
    title?: string
    createdAt: string
    commentCount: number
    rootCommentCount: number
    /** 이를 null로 설정하면 모든 SSO 사용자가 페이지를 볼 수 있습니다. 빈 리스트는 모든 사용자에게 닫혀 있음을 의미합니다. **/
    accessibleByGroupIds?: string[] | null
    /** 이 페이지가 새 댓글 작성에 대해 닫혀 있나요? **/
    isClosed?: boolean
}
[inline-code-end]

---
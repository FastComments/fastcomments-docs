`FeedPost` 객체는 FastComments 피드의 게시물을 나타냅니다. 피드는 자체 댓글 스레드를 가진 게시물 스트림으로, Feed 위젯에 의해 렌더링됩니다. 각 게시물은 작성자, 선택적 풍부 콘텐츠, 미디어 및 링크를 가지고 있으며, 피드를 필터링할 수 있도록 태그를 지정할 수 있습니다.

`FeedPost` 객체의 구조는 다음과 같습니다:

[inline-code-attrs-start title = 'FeedPost 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPost {
    /** 읽기 전용 **/
    _id: string
    /** 읽기 전용 **/
    tenantId: string
    title?: string
    /** 게시물을 작성한 FastComments 또는 SSO 사용자의 ID. **/
    fromUserId?: string
    /** 설정되지 않은 경우 사용자가 채웁니다. **/
    fromUserDisplayName?: string | null
    /** 읽기 전용. 사용자가 채웁니다. **/
    fromUserAvatar?: string | null
    /** 피드를 필터링하는 데 사용됩니다. **/
    tags?: string[]
    /** 피드 내 정렬 가중치. 값이 높을수록 먼저 정렬됩니다. **/
    weight?: number
    /** 사용자를 위한 자유 형식 키/값 쌍. **/
    meta?: Record<string, string>
    /** 정제된 HTML. **/
    contentHTML?: string
    media?: FeedPostMediaItem[]
    links?: FeedPostLink[]
    /** 읽기 전용 **/
    createdAt: string
    /** 읽기 전용. 카운트할 반응 유형. **/
    reacts?: Record<string, number>
    /** 읽기 전용 **/
    commentCount?: number | null
}

interface FeedPostMediaItem {
    title?: string
    /** 클릭 시 미디어 항목이 연결되는 위치. **/
    linkUrl?: string
    /** 렌더링당 하나의 항목. 위젯이 가장 적합한 것을 선택합니다. **/
    sizes: FeedPostMediaItemAsset[]
}

interface FeedPostMediaItemAsset {
    w: number
    h: number
    src: string
}

interface FeedPostLink {
    /** 링크 텍스트, 예: "지금 가입". **/
    text?: string
    /** 링크와 함께 표시되는 헤딩. **/
    title?: string
    /** 링크와 함께 표시되는 설명. **/
    description?: string
    url?: string
}
[inline-code-end]

참고:

- `READONLY` 로 표시된 필드가 있습니다 - 이 필드들은 API에서 반환되지만 설정할 수 없습니다.
- 게시물에 대한 댓글은 `urlId`가 `post:` 뒤에 게시물 `_id`가 붙은 일반 댓글입니다. 해당 값을 Comment API와 함께 사용하여 게시물에 대한 댓글을 읽거나 생성할 수 있습니다.

---
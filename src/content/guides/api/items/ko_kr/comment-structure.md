A `Comment` object represents a comment left by a user.

The relationship between parent and child comments is defined via `parentId`.

The structure for the Comment object is as follows:

[inline-code-attrs-start title = '댓글 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Comment {
    /** READONLY: 스팸 엔진이 댓글을 스팸으로 판정한 경우 true로 설정됩니다. **/
    aiDeterminedSpam?: boolean
    /** 댓글이 표시되도록 승인되었는지 여부. 댓글 저장 시 true로 설정되며, 그렇지 않으면 숨겨집니다. **/
    approved?: boolean
    /** 사용자의 아바타. **/
    avatarSrc?: string
    /** 하위 댓글. 모든 상황에서 채워지지는 않습니다. API를 통해 asTree가 true로 설정된 경우 사용됩니다. **/
    children: Comment[]
    /** 작성자가 입력한 원본 댓글. **/
    comment: string
    /** READONLY: 작성자의 댓글을 HTML로 파싱한 결과. **/
    commentHTML?: string
    /** 작성자의 이메일. 익명 댓글 허용이 꺼져 있으면 필요합니다. **/
    commenterEmail?: string
    /** 작성자의 링크(예: 블로그). **/
    commenterLink?: string
    /** 작성자의 이름. 항상 필요합니다. 사용 불가능한 경우 'Anonymous' 같은 값을 설정하세요. **/
    commenterName: string
    /** 댓글이 남겨진 날짜(UTC epoch). **/
    date: number
    /** 댓글의 '표시 레이블' — 예: 'Admin', 'Moderator' 또는 'VIP User'와 같은 값. **/
    displayLabel?: string
    /** 댓글이 게시된 도메인. **/
    domain?: string
    /** READONLY: 댓글이 신고된 횟수. **/
    flagCount?: number
    /** 댓글에 작성되어 성공적으로 파싱된 #해시태그들. 검색용으로 해시태그를 수동으로 추가할 수도 있지만, 자동으로 댓글 본문에 표시되지는 않습니다. **/
    hashTags?: CommentHashTag[]
    /** READONLY: 댓글에 이미지가 포함되어 있는지 여부. **/
    hasImages?: boolean
    /** READONLY: 댓글에 링크가 포함되어 있는지 여부. **/
    hasLinks?: boolean
    /** READONLY: 고유 댓글 ID. **/
    id: string
    /** 생성 시에만 사용됩니다! 저장을 위해 해시 처리됩니다. **/
    ip?: string
    /** READONLY: 현재 사용자가 이 댓글을 작성한 사용자를 차단했는지 여부. **/
    isBlocked?: boolean
    /** READONLY: 댓글 작성자가 관리자(admin)인지 여부. userId를 기반으로 자동 설정됩니다. **/
    isByAdmin?: boolean
    /** READONLY: 댓글 작성자가 중재자(moderator)인지 여부. userId를 기반으로 자동 설정됩니다. **/
    isByModerator?: boolean
    /** 댓글이 소프트 삭제된 경우 true로 설정됩니다(다른 구성으로 인해 자리 표시자가 남겨져야 했던 경우). **/
    isDeleted?: boolean
    /** 사용자의 계정이 삭제되어 댓글을 유지해야 했던 경우 true로 설정됩니다. **/
    isDeletedUser?: boolean
    /** READONLY: 현재 로그인한 사용자(contextUserId)가 이 댓글을 신고했는지 여부. **/
    isFlagged?: boolean
    /** 댓글이 고정되었는지 여부. **/
    isPinned?: boolean
    /** 댓글이 잠겼는지 여부. true이면 잠금 해제될 때까지 누구(중재자 포함)도 답글, 편집 또는 삭제를 할 수 없습니다. **/
    isLocked?: boolean
    /** 댓글이 스팸인지 여부. **/
    isSpam?: boolean
    /** READONLY: 현재 사용자(contextUserId)가 이 댓글에 대해 다운보트했는지 여부. **/
    isVotedDown?: boolean
    /** READONLY: 현재 사용자(contextUserId)가 이 댓글에 대해 업보트했는지 여부. **/
    isVotedUp?: boolean
    /** 댓글의 로케일. 제공되지 않으면 HTTP의 Accept-Language 헤더에서 파생됩니다. **/
    locale?: 'de_de' | 'en_us' | 'es_es' | 'fr_fr' | 'it_it' | 'ja_jp' | 'ko_kr' | 'pl_pl' | 'pt_br' | 'ru_ru' | 'tr_tr' | 'zh_cn' | 'zh_tw'
    /** READONLY: 댓글에 작성되어 성공적으로 파싱된 @멘션들. **/
    mentions?: CommentUserMention[]
    /** 댓글에 연결된 선택적 메타데이터. **/
    meta?: Record<string, string | number | boolean>
    /** 이 댓글과 연관된 선택적 중재 그룹 ID 목록. **/
    moderationGroupIds?: string[]|null
    /** READONLY: 현재 사용자(contextUserId)가 이 댓글에 한 투표에 해당하는 투표 객체의 ID. **/
    myVoteId?: string
    /** 댓글 작성자에게 이 댓글에 대한 알림이 전송되었는지 여부. 가져오기(imports) 시 알림 전송을 방지하려면 이 값을 true로 설정하세요. **/
    notificationSentForParent?: boolean
    /** 테넌트 사용자에게 이 댓글에 대한 알림이 전송되었는지 여부. 가져오기 시 알림 전송을 방지하려면 이 값을 true로 설정하세요. **/
    notificationSentForParentTenant?: boolean
    /** 이 댓글이 달린 페이지의 제목. **/
    pageTitle?: string
    /** 답글일 경우 응답 대상 댓글의 ID입니다. **/
    parentId?: string|null
    /** 댓글이 검토됨으로 표시되었는지 여부. **/
    reviewed: boolean
    /** 댓글이 속한 테넌트 ID. **/
    tenantId: string
    /** 댓글을 작성한 사용자. 이름/이메일과 함께 댓글을 저장할 때 자동으로 생성됩니다. **/
    userId?: string|null
    /** 이 댓글이 표시되는 위치의 URL(예: 블로그 게시물). **/
    url: string
    /** 전달한 urlId의 '정리된' 버전. 저장 시 이 필드를 지정하지만, 댓글을 가져오면 이 값은 '정리'되어 원래 값은 urlIdRaw로 이동합니다. **/
    urlId: string
    /** READONLY: 전달한 원래의 urlId. **/
    urlIdRaw?: string
    /** 사용자와 이 댓글이 검증되었는지 여부. **/
    verified: boolean
    /** 업보트 수. **/
    votesUp?: number
    /** 다운보트 수. **/
    votesDown?: number
    /** 댓글의 '카르마'(= 업보트 - 다운보트). **/
    votes?: number
}
[inline-code-end]

Some of these fields are marked `READONLY` - these are returned by the API but cannot be set.

### 댓글 텍스트 구조

Comments are written in a FastComments flavor of markdown, which is just markdown plus traditional `bbcode` style tags for images, like `[img]path[/img]`.

텍스트는 두 필드에 저장됩니다. 사용자가 입력한 텍스트는 수정되지 않은 채 `comment` 필드에 저장됩니다. 렌더링된 결과는 `commentHTML` 필드에 저장됩니다.

The allowed HTML tags are `b, u, i, strike, pre, span, code, img, a, strong, ul, ol, li, and br`.

HTML은 아주 작은 부분집합이므로 HTML을 렌더링하는 것을 권장합니다. 렌더러를 만드는 것은 비교적 간단합니다. 예를 들어 React Native와 Flutter용으로 이를 도와주는 여러 라이브러리가 있습니다

You may choose to render the un-normalized value of the `comment` field. [An example parser is here.](https://github.com/FastComments/fastcomments-code-examples/blob/master/custom-client/client/parse-comment.js).

예제 파서는 HTML에 맞게 조정하여 HTML 태그를 플랫폼에 맞는 렌더링 요소로 변환하도록 할 수도 있습니다. 

### 태깅

When users are tagged in a comment, the information is stored in a list called `mentions`. Each object in that list
has the following structure.

[inline-code-attrs-start title = '댓글 멘션 객체'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** 사용자 ID. SSO 사용자인 경우 테넌트 ID가 접두사로 붙습니다. **/
    id: string
    /** 최종 @멘션 태그 텍스트( @ 기호 포함). **/
    tag: string
    /** 원본 @멘션 태그 텍스트( @ 기호 포함). **/
    rawTag: string
    /** 어떤 유형의 사용자가 태그되었는지. user = FastComments.com 계정, sso = SSO 사용자. **/
    type: 'user'|'sso'
    /** 사용자가 알림 수신을 거부한 경우에도 이 값은 true로 설정됩니다. **/
    sent: boolean
}
[inline-code-end]

### 해시태그

When hashtags are used and successfully parsed, the information is stored in a list called `hashTags`. Each object in that list
has the following structure. Hashtags can also be manually added to the comment `hashTags` array for querying, if `retain` is set.

[inline-code-attrs-start title = '댓글 해시태그 객체'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentHashTag {
    /** 해시태그 ID. **/
    id: string
    /** 최종 #해시태그 태그 텍스트(# 기호 포함). **/
    tag: string
    /** 해시태그가 커스텀 URL과 연결되어 있으면 이 값이 정의됩니다. **/
    url?: string
    /** 댓글을 업데이트할 때 댓글 본문에 존재하지 않더라도 해시태그를 유지할지 여부. 댓글 본문을 변경하지 않고 댓글에 태그를 달 때 유용합니다. **/
    retain?: boolean
}
[inline-code-end]

---
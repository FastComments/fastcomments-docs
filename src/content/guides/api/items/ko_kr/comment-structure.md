A `Comment` 객체는 사용자가 남긴 댓글을 나타냅니다.

상위 및 하위 댓글 간의 관계는 `parentId`를 통해 정의됩니다.

Comment 객체의 구조는 다음과 같습니다:

[inline-code-attrs-start title = '댓글 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Comment {
    /** 읽기 전용: 스팸 엔진이 해당 댓글을 스팸으로 판별한 경우 true로 설정됩니다. **/
    aiDeterminedSpam?: boolean
    /** 댓글이 표시되도록 승인되었는지 여부. 댓글 저장 시 true로 설정되며, 그렇지 않으면 숨겨집니다. **/
    approved?: boolean
    /** 사용자의 아바타. **/
    avatarSrc?: string
    /** 자식 댓글. 모든 시나리오에서 채워지지는 않습니다. API를 통해 asTree가 true로 설정된 경우 사용됩니다. **/
    children: Comment[]
    /** 작성자가 입력한 원본 댓글 텍스트. **/
    comment: string
    /** 읽기 전용: 작성자의 댓글을 파싱해 HTML로 변환한 값. **/
    commentHTML?: string
    /** 작성자의 이메일. 익명 댓글이 꺼져 있으면 필요합니다. **/
    commenterEmail?: string
    /** 작성자의 링크(예: 블로그). **/
    commenterLink?: string
    /** 작성자의 이름. 항상 필요합니다. 사용할 수 없다면 "Anonymous" 같은 값으로 설정하세요. **/
    commenterName: string
    /** 댓글이 작성된 날짜(UTC epoch). **/
    date: number
    /** 댓글의 "표시 레이블" - 예: "Admin", "Moderator" 또는 "VIP User" 같은 값. **/
    displayLabel?: string
    /** 댓글이 게시된 도메인. **/
    domain?: string
    /** 읽기 전용: 댓글이 신고된 횟수. **/
    flagCount?: number
    /** 댓글에 작성되어 성공적으로 파싱된 #해시태그들. 쿼리를 위해 해시태그를 수동으로 추가할 수도 있지만, 그러면 댓글 텍스트에 자동으로 표시되지는 않습니다. **/
    hashTags?: CommentHashTag[]
    /** 읽기 전용: 댓글에 이미지가 포함되어 있는지 여부. **/
    hasImages?: boolean
    /** 읽기 전용: 댓글에 링크가 포함되어 있는지 여부. **/
    hasLinks?: boolean
    /** 읽기 전용: 고유 댓글 ID. **/
    id: string
    /** 생성 시에만! 저장을 위해 해시됩니다. **/
    ip?: string
    /** 읽기 전용: 현재 사용자가 이 댓글 작성자를 차단했는지 여부. **/
    isBlocked?: boolean
    /** 읽기 전용: 댓글 작성자가 관리자인지 여부. userId를 기반으로 자동 설정됩니다. **/
    isByAdmin?: boolean
    /** 읽기 전용: 댓글 작성자가 중재자인지 여부. userId를 기반으로 자동 설정됩니다. **/
    isByModerator?: boolean
    /** 댓글이 소프트 삭제된 경우(true로 설정됩니다). (다른 구성 때문에 플레이스홀더를 남겨야 하는 경우) **/
    isDeleted?: boolean
    /** 사용자의 계정이 삭제되어 댓글만 보존해야 하는 경우 true로 설정됩니다. **/
    isDeletedUser?: boolean
    /** 읽기 전용: 현재 로그인한 사용자(contextUserId)가 이 댓글을 신고했는지 여부. **/
    isFlagged?: boolean
    /** 댓글이 고정되어 있는지 여부. **/
    isPinned?: boolean
    /** 새 답글 작성이 잠겨 있는지 여부(중재자는 여전히 답글 작성 가능). **/
    isLocked?: boolean
    /** 댓글이 스팸인지 여부. **/
    isSpam?: boolean
    /** 읽기 전용: 현재 사용자(contextUserId)가 이 댓글에 대해 downvote했는지 여부. **/
    isVotedDown?: boolean
    /** 읽기 전용: 현재 사용자(contextUserId)가 이 댓글에 대해 upvote했는지 여부. **/
    isVotedUp?: boolean
    /** 댓글의 로케일. 제공되지 않으면 HTTP Accept-Language 헤더에서 파생됩니다. **/
    locale?: 'de_de' | 'en_us' | 'es_es' | 'fr_fr' | 'it_it' | 'ja_jp' | 'ko_kr' | 'pl_pl' | 'pt_br' | 'ru_ru' | 'tr_tr' | 'zh_cn' | 'zh_tw'
    /** 읽기 전용: 댓글에서 성공적으로 파싱된 @멘션들. **/
    mentions?: CommentUserMention[]
    /** 댓글에 연결된 선택적 메타데이터. **/
    meta?: Record<string, string | number | boolean>
    /** 이 댓글에 연결된 선택적 중재 그룹 ID 목록. **/
    moderationGroupIds?: string[]|null
    /** 읽기 전용: 현재 사용자(contextUserId)가 이 댓글에 남긴 투표에 해당하는 투표 객체의 ID. **/
    myVoteId?: string
    /** 이 댓글에 대해 작성자들에게 알림이 전송되었는지 여부. 가져오기(imports) 시 알림 전송을 방지하려면 이 값을 true로 설정하세요. **/
    notificationSentForParent?: boolean
    /** 이 댓글에 대해 테넌트 사용자들에게 알림이 전송되었는지 여부. 가져오기 시 알림 전송을 방지하려면 이 값을 true로 설정하세요. **/
    notificationSentForParentTenant?: boolean
    /** 이 댓글이 작성된 페이지의 제목. **/
    pageTitle?: string
    /** 댓글에 답글을 다는 경우, 답글 대상 댓글의 ID입니다. **/
    parentId?: string|null
    /** 댓글이 검토됨으로 표시되었는지 여부. **/
    reviewed: boolean
    /** 댓글이 속한 테넌트 ID. **/
    tenantId: string
    /** 댓글을 작성한 사용자. 이름/이메일로 댓글을 저장하면 자동으로 생성됩니다. **/
    userId?: string|null
    /** 이 댓글이 표시되는 위치의 URL(예: 블로그 게시물). **/
    url: string
    /** 전달한 urlId의 "정리된" 버전. 저장 시에는 이 필드를 지정하지만, 댓글을 가져올 때 이 값은 정리되어 원래 값은 "urlIdRaw"로 이동합니다. **/
    urlId: string
    /** 읽기 전용: 전달한 원래 urlId. **/
    urlIdRaw?: string
    /** 사용자와 이 댓글이 인증되었는지 여부. **/
    verified: boolean
    /** 찬성(업) 투표 수. **/
    votesUp?: number
    /** 반대(다운) 투표 수. **/
    votesDown?: number
    /** 댓글의 "카르마"(= 찬성 수 - 반대 수). **/
    votes?: number
}
[inline-code-end]

일부 필드는 `READONLY`로 표시되어 있습니다 — 이들 필드는 API에서 반환되지만 설정할 수 없습니다.

### 댓글 텍스트 구조

댓글은 FastComments 변형 마크다운으로 작성됩니다. 이는 일반 마크다운에 전통적인 `bbcode` 스타일의 이미지 태그(예: `[img]path[/img]`)를 추가한 것입니다.

텍스트는 두 필드에 저장됩니다. 사용자가 입력한 원문 텍스트는 `comment` 필드에 변경 없이 저장됩니다. 이 텍스트는 렌더링되어 `commentHTML` 필드에 저장됩니다.

허용되는 HTML 태그는 `b, u, i, strike, pre, span, code, img, a, strong, ul, ol, li, and br`입니다.

허용되는 HTML이 매우 적은 하위 집합이므로 HTML을 렌더링하는 것이 권장됩니다. 렌더러를 만드는 것은 비교적 간단합니다. 예를 들어 React Native와 Flutter용 라이브러리가 여러 개 있습니다.

정규화되지 않은 `comment` 필드의 값을 렌더링하도록 선택할 수도 있습니다. [예제 파서가 여기 있습니다.](https://github.com/FastComments/fastcomments-code-examples/blob/master/custom-client/client/parse-comment.js).

예제 파서는 HTML과 함께 작동하도록 조정할 수 있으며, HTML 태그를 플랫폼에서 렌더링할 예상 요소로 변환할 수 있습니다.

### 태깅

댓글에서 사용자가 태그되면 해당 정보는 `mentions`라는 리스트에 저장됩니다. 해당 리스트의 각 객체는 다음 구조를 가집니다.

[inline-code-attrs-start title = '댓글 멘션 객체'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** 사용자 ID. SSO 사용자의 경우 테넌트 ID가 접두사로 붙습니다. **/
    id: string
    /** 최종 @멘션 태그 텍스트(@ 기호 포함). **/
    tag: string
    /** 원래의 @멘션 태그 텍스트(@ 기호 포함). **/
    rawTag: string
    /** 태그된 사용자의 유형. user = FastComments.com 계정, sso = SSO 사용자. **/
    type: 'user'|'sso'
    /** 사용자가 알림을 수신 거부하더라도 이 값은 여전히 true로 설정됩니다. **/
    sent: boolean
}
[inline-code-end]

### 해시태그

해시태그가 사용되어 성공적으로 파싱되면 그 정보는 `hashTags`라는 리스트에 저장됩니다. 해당 리스트의 각 객체는 다음과 같은 구조를 가집니다. `retain`이 설정되어 있으면 해시태그를 쿼리를 위해 댓글의 `hashTags` 배열에 수동으로 추가할 수도 있습니다.

[inline-code-attrs-start title = '댓글 해시태그 객체'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentHashTag {
    /** 해시태그 ID. **/
    id: string
    /** 최종 #해시태그 텍스트(# 기호 포함). **/
    tag: string
    /** 해시태그에 맞춤 URL이 연결되어 있다면 이 값이 정의됩니다. **/
    url?: string
    /** 댓글이 업데이트될 때 댓글 텍스트에 존재하지 않더라도 해시태그를 유지(retain)해야 하는지 여부. 댓글 텍스트를 변경하지 않고 댓글에 태그를 추가할 때 유용합니다. **/
    retain?: boolean
}
[inline-code-end]

---
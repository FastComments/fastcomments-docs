`Comment` 객체는 사용자가 남긴 댓글을 나타냅니다.

부모 댓글과 자식 댓글 간의 관계는 `parentId`를 통해 정의됩니다.

Comment 객체의 구조는 다음과 같습니다:

[inline-code-attrs-start title = 'Comment 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Comment {
    /** READONLY: 스팸 엔진이 해당 댓글을 스팸으로 판단한 경우 true로 설정됩니다. **/
    aiDeterminedSpam?: boolean
    /** 댓글이 표시되도록 승인되었는지 여부입니다. 댓글 저장 시 true로 설정하지 않으면 숨겨집니다. **/
    approved?: boolean
    /** 사용자의 아바타입니다. **/
    avatarSrc?: string
    /** 자식 댓글들입니다. 모든 시나리오에서 채워지는 것은 아닙니다. API를 통해 asTree가 true로 설정된 경우 사용됩니다. **/
    children: Comment[]
    /** 댓글 작성자의 원본 댓글입니다. **/
    comment: string
    /** READONLY: 댓글 작성자의 댓글이 HTML로 파싱된 형태입니다. **/
    commentHTML?: string
    /** 댓글 작성자의 이메일입니다. 익명 댓글 작성이 비활성화된 경우 필수입니다. **/
    commenterEmail?: string
    /** 댓글 작성자의 링크(예: 자신의 블로그)입니다. **/
    commenterLink?: string
    /** 댓글 작성자의 이름입니다. 항상 필수입니다. 사용할 수 없는 경우 "Anonymous"와 같은 값으로 설정하세요. **/
    commenterName: string
    /** 댓글이 작성된 날짜로, UTC epoch 형식입니다. **/
    date: number
    /** 댓글의 "표시 라벨"입니다 - 예를 들어 "Admin", "Moderator" 또는 "VIP User" 같은 값입니다. **/
    displayLabel?: string
    /** 댓글이 작성된 도메인입니다. **/
    domain?: string
    /** READONLY: 댓글이 신고된 횟수입니다. **/
    flagCount?: number
    /** 댓글에 작성되어 성공적으로 파싱된 #해시태그입니다. 쿼리용으로 해시태그를 수동으로 추가할 수도 있지만, 댓글 텍스트에는 자동으로 표시되지 않습니다. **/
    hashTags?: CommentHashTag[]
    /** READONLY: 댓글에 이미지가 포함되어 있습니까? **/
    hasImages?: boolean
    /** READONLY: 댓글에 링크가 포함되어 있습니까? **/
    hasLinks?: boolean
    /** READONLY: 고유한 댓글 id입니다. **/
    id: string
    /** 생성 시에만 사용됩니다! 저장 시 해시됩니다. **/
    ip?: string
    /** READONLY: 현재 사용자가 이 댓글을 작성한 사용자를 차단했습니까? **/
    isBlocked?: boolean
    /** READONLY: 관리자가 작성한 댓글입니까? userId를 기준으로 자동 설정됩니다. **/
    isByAdmin?: boolean
    /** READONLY: 중재자가 작성한 댓글입니까? userId를 기준으로 자동 설정됩니다. **/
    isByModerator?: boolean
    /** 댓글이 소프트 삭제된 경우 true로 설정됩니다(다른 설정으로 인해 플레이스홀더를 남겨야 하는 경우). **/
    isDeleted?: boolean
    /** 사용자의 계정이 삭제되었고 댓글을 보존해야 하는 경우 true로 설정됩니다. **/
    isDeletedUser?: boolean
    /** READONLY: 현재 로그인한 사용자(contextUserId)가 신고했습니까? **/
    isFlagged?: boolean
    /** 댓글이 고정되어 있습니까? **/
    isPinned?: boolean
    /** 댓글이 잠겨 있습니까? true인 경우 잠금이 해제될 때까지 누구도(중재자 포함) 답글을 달거나, 편집하거나, 삭제할 수 없습니다. **/
    isLocked?: boolean
    /** 댓글이 스팸입니까? **/
    isSpam?: boolean
    /** READONLY: 현재 사용자(contextUserId)가 이 댓글에 싫어요를 투표했습니까? **/
    isVotedDown?: boolean
    /** READONLY: 현재 사용자(contextUserId)가 이 댓글에 좋아요를 투표했습니까? **/
    isVotedUp?: boolean
    /** 댓글의 로케일입니다. 제공되지 않으면 language accept HTTP 헤더에서 파생됩니다. **/
    locale?: 'de_de' | 'en_us' | 'es_es' | 'fr_fr' | 'it_it' | 'ja_jp' | 'ko_kr' | 'pl_pl' | 'pt_br' | 'ru_ru' | 'tr_tr' | 'zh_cn' | 'zh_tw'
    /** READONLY: 댓글에 작성되어 성공적으로 파싱된 @멘션입니다. **/
    mentions?: CommentUserMention[]
    /** 댓글과 연관된 선택적 메타데이터입니다. **/
    meta?: Record<string, string | number | boolean>
    /** 이 댓글과 연관된 중재 그룹 id의 선택적 목록입니다. **/
    moderationGroupIds?: string[]|null
    /** READONLY: 현재 사용자(contextUserId)가 이 댓글에 한 투표에 해당하는 투표 객체의 id입니다. **/
    myVoteId?: string
    /** 이 댓글에 대해 댓글 작성자들에게 알림이 전송되었는지 여부입니다. 가져오기 시 알림 전송을 방지하려면 true로 설정하세요. **/
    notificationSentForParent?: boolean
    /** 이 댓글에 대해 테넌트 사용자들에게 알림이 전송되었는지 여부입니다. 가져오기 시 알림 전송을 방지하려면 true로 설정하세요. **/
    notificationSentForParentTenant?: boolean
    /** 이 댓글이 있던 페이지의 제목입니다. **/
    pageTitle?: string
    /** 댓글에 답장하는 경우, 답장 대상 댓글의 ID입니다. **/
    parentId?: string|null
    /** 댓글이 검토 완료로 표시되었는지 여부입니다. **/
    reviewed: boolean
    /** 댓글이 속한 테넌트 id입니다. **/
    tenantId: string
    /** 댓글을 작성한 사용자입니다. 이름/이메일과 함께 댓글을 저장할 때 자동으로 생성됩니다. **/
    userId?: string|null
    /** 이 댓글이 표시되는 위치(예: 블로그 게시물)의 URL입니다. **/
    url: string
    /** 전달하신 urlId의 "정리된" 버전입니다. 저장 시 이 필드를 지정하지만, 댓글을 다시 가져올 때는 "정리된" 값이 되며 원래 값은 "urlIdRaw"로 이동됩니다. **/
    urlId: string
    /** READONLY: 전달하신 원래 urlId입니다. **/
    urlIdRaw?: string
    /** 사용자와 이 댓글이 인증되었습니까? **/
    verified: boolean
    /** 좋아요 수입니다. **/
    votesUp?: number
    /** 싫어요 수입니다. **/
    votesDown?: number
    /** 댓글의 "카르마"입니다(= 좋아요 수 - 싫어요 수). **/
    votes?: number
}
[inline-code-end]

이 필드 중 일부는 `READONLY`로 표시되어 있습니다 - 이는 API에서 반환되지만 설정할 수 없습니다.

### 댓글 텍스트 구조

댓글은 FastComments 방식의 마크다운으로 작성되며, 이는 일반 마크다운에 `[img]path[/img]` 같은 이미지용 전통적인 `bbcode` 스타일 태그를 더한 것입니다.

텍스트는 두 필드에 저장됩니다. 사용자가 입력한 텍스트는 수정되지 않은 상태로 `comment` 필드에 저장됩니다. 이것이 렌더링되어 `commentHTML` 필드에 저장됩니다.

허용되는 HTML 태그는 `b, u, i, strike, pre, span, code, img, a, strong, ul, ol, li, and br`입니다.

HTML을 렌더링하는 것이 권장됩니다. 매우 작은 HTML 하위 집합이므로 렌더러를 만드는 것은 꽤 간단합니다. 예를 들어 React Native와 Flutter용으로 이를 도와주는 여러 라이브러리가 있습니다.

`comment` 필드의 정규화되지 않은 값을 렌더링하도록 선택할 수 있습니다. [예제 파서는 여기에 있습니다.](https://github.com/FastComments/fastcomments-code-examples/blob/master/custom-client/client/parse-comment.js).

예제 파서는 HTML과 함께 작동하도록 조정하여 HTML 태그를 플랫폼에 맞는 렌더링 요소로 변환할 수도 있습니다.

### 태깅

댓글에서 사용자가 태그되면 해당 정보는 `mentions`라는 목록에 저장됩니다. 해당 목록의 각 객체는
다음과 같은 구조를 가집니다.

[inline-code-attrs-start title = 'Comment 멘션 객체'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** 사용자 id입니다. SSO 사용자의 경우 테넌트 id가 접두사로 붙습니다. **/
    id: string
    /** @ 기호를 포함한 최종 @멘션 태그 텍스트입니다. **/
    tag: string
    /** @ 기호를 포함한 원래 @멘션 태그 텍스트입니다. **/
    rawTag: string
    /** 태그된 사용자 유형입니다. user = FastComments.com 계정. sso = SSOUser. **/
    type: 'user'|'sso'
    /** 사용자가 알림을 거부한 경우에도 이 값은 true로 설정됩니다. **/
    sent: boolean
}
[inline-code-end]

### 해시태그

해시태그가 사용되어 성공적으로 파싱되면 해당 정보는 `hashTags`라는 목록에 저장됩니다. 해당 목록의 각 객체는
다음과 같은 구조를 가집니다. `retain`이 설정된 경우, 쿼리용으로 댓글의 `hashTags` 배열에 해시태그를 수동으로 추가할 수도 있습니다.

[inline-code-attrs-start title = 'Comment 해시태그 객체'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentHashTag {
    /** 해시태그 id입니다. **/
    id: string
    /** # 기호를 포함한 최종 #해시태그 태그 텍스트입니다. **/
    tag: string
    /** 해시태그가 사용자 지정 URL과 연관된 경우 이 값이 정의됩니다. **/
    url?: string
    /** 댓글이 업데이트될 때 해시태그가 댓글 텍스트에 존재하지 않더라도 유지해야 하는 경우 설정합니다. 댓글 텍스트를 변경하지 않고 댓글에 태그를 지정하는 데 유용합니다. **/
    retain?: boolean
}
[inline-code-end]
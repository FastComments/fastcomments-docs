웹훅을 통해 전송되는 유일한 구조는 아래에 TypeScript로 정리된 WebhookComment 객체입니다.

#### WebhookComment 객체 구조

##### "Create" 이벤트 구조
"create" 이벤트의 요청 본문은 WebhookComment 객체입니다.

##### "Update" 이벤트 구조
"update" 이벤트의 요청 본문은 WebhookComment 객체입니다.

##### "Delete" 이벤트 구조
"delete" 이벤트의 요청 본문은 WebhookComment 객체입니다.

    2023년 11월 14일 기준 변경 사항
    이전에는 "delete" 이벤트의 요청 본문에 댓글 id만 포함되어 있었습니다. 이제는 삭제 시점의 전체 댓글이 포함됩니다.


[inline-code-attrs-start title = 'WebhookComment 객체'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** 댓글의 id. **/
    id: string
    /** 댓글 스레드를 식별하는 id 또는 URL. 정규화됨. **/
    urlId: string
    /** 댓글이 남겨진 위치를 가리키는 URL. **/
    url?: string
    /** 댓글을 작성한 사용자의 id. SSO의 경우 테넌트 id가 접두사로 붙습니다. **/
    userId?: string
    /** 댓글을 남긴 사용자의 이메일. **/
    commenterEmail?: string
    /** 댓글 위젯에 표시되는 사용자 이름. SSO의 경우 displayName일 수 있습니다. **/
    commenterName: string
    /** 원본 댓글 텍스트. **/
    comment: string
    /** 파싱된 후의 댓글 텍스트. **/
    commentHTML: string
    /** 댓글 외부 id. **/
    externalId?: string
    /** 부모 댓글의 id. **/
    parentId?: string | null
    /** 댓글이 작성된 UTC 날짜. **/
    date: UTC_ISO_DateString
    /** 투표의 총 합계(찬성 - 반대). **/
    votes: number
    votesUp: number
    votesDown: number
    /** 댓글 작성 시 사용자가 로그인되어 있었거나, 댓글을 검증했거나, 댓글 작성 시 세션을 검증한 경우 true. **/
    verified: boolean
    /** 댓글이 검증된 날짜. **/
    verifiedDate?: number
    /** 중재자가 댓글을 검토됨으로 표시했는지 여부. **/
    reviewed: boolean
    /** 아바타의 위치 또는 base64 인코딩. SSO로 전달된 값이 base64인 경우에만 base64가 됩니다. **/
    avatarSrc?: string
    /** 댓글이 수동 또는 자동으로 스팸으로 표시되었는지 여부. **/
    isSpam: boolean
    /** 댓글이 자동으로 스팸으로 판정되었는지 여부. **/
    aiDeterminedSpam: boolean
    /** 댓글에 이미지가 포함되어 있는지 여부. **/
    hasImages: boolean
    /** "Most Relevant" 정렬 방식에서 댓글이 위치한 페이지 번호. **/
    pageNumber: number
    /** "Oldest First" 정렬 방식에서 댓글이 위치한 페이지 번호. **/
    pageNumberOF: number
    /** "Newest First" 정렬 방식에서 댓글이 위치한 페이지 번호. **/
    pageNumberNF: number
    /** 댓글이 자동으로 승인되었는지 또는 수동으로 승인되었는지 여부. **/
    approved: boolean
    /** 댓글 작성 시 사용자의 로케일 코드(형식: en_us). **/
    locale: string
    /** 댓글에 작성된, 성공적으로 파싱된 @멘션들. **/
    mentions?: CommentUserMention[]
    /** 댓글이 작성된 도메인. **/
    domain?: string
    /** 이 댓글과 연관된 선택적 중재 그룹 id 목록. **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

When users are tagged in a comment, the information is stored in a list called `mentions`. Each object in that list
has the following structure.

[inline-code-attrs-start title = 'Webhook 멘션 객체'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** 사용자 id. SSO 사용자인 경우 테넌트 id가 접두사로 붙습니다. **/
    id: string
    /** @ 기호를 포함한 최종 @멘션 태그 텍스트. **/
    tag: string
    /** @ 기호를 포함한 원래의 @멘션 태그 텍스트. **/
    rawTag: string
    /** 태그된 사용자의 유형. user = FastComments.com 계정. sso = SSO 사용자. **/
    type: 'user'|'sso'
    /** 사용자가 알림을 거부해도 이 값은 여전히 true로 설정됩니다. **/
    sent: boolean
}
[inline-code-end]

#### 사용된 HTTP 메서드

**Create와 Update는 둘 다 HTTP PUT을 사용하며 POST를 사용하지 않습니다!**

모든 요청에 ID가 포함되어 있으므로 동일한 Create 또는 Update 요청을 반복해도 새 객체가 생성되지 않아야 합니다.

이는 이러한 호출이 멱등(idempotent)하며 HTTP 명세에 따라 PUT 이벤트여야 함을 의미합니다.
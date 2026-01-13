`UserBadge`는 FastComments 시스템에서 사용자에게 할당된 배지를 나타내는 객체입니다.

배지는 활동(예: 댓글 수, 응답 시간, 베테랑 상태)에 따라 자동으로 사용자에게 할당되거나 사이트 관리자가 수동으로 할당할 수 있습니다.

The structure for the `UserBadge` object is as follows:

[inline-code-attrs-start title = 'UserBadge 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface UserBadge {
    /** 이 사용자 배지 할당의 고유 식별자 */
    id: string
    /** 이 배지가 할당된 사용자의 ID */
    userId: string
    /** 테넌트의 배지 카탈로그에 있는 배지 정의의 ID */
    badgeId: string
    /** 이 배지를 생성/할당한 테넌트의 ID */
    fromTenantId: string
    /** 이 배지가 생성된 시각 (에포크 이후 밀리초) */
    createdAt?: number
    /** 사용자가 이 배지를 받은 시각 (에포크 이후 밀리초) */
    receivedAt?: number
    /** 
     * 배지 유형: 
     * 0=CommentCount, 1=CommentUpVotes, 2=CommentReplies, 3=CommentsPinned, 
     * 4=Veteran, 5=NightOwl, 6=FastReplyTime, 7=ModeratorCommentsDeleted,
     * 8=ModeratorCommentsApproved, 9=ModeratorCommentsUnapproved, 10=ModeratorCommentsReviewed,
     * 11=ModeratorCommentsMarkedSpam, 12=ModeratorCommentsMarkedNotSpam, 13=RepliedToSpecificPage,
     * 14=Manual
     */
    type: number
    /** 임계값 기반 배지의 임계값 */
    threshold?: number
    /** 배지의 이름/레이블 */
    name?: string
    /** 배지에 대한 자세한 설명 */
    description?: string
    /** 배지에 표시되는 텍스트 */
    displayLabel?: string
    /** 배지에 표시되는 이미지의 URL */
    displaySrc?: string
    /** 배지의 배경색 (헥스 코드) */
    backgroundColor?: string
    /** 배지의 테두리 색상 (헥스 코드) */
    borderColor?: string
    /** 배지의 텍스트 색상 (헥스 코드) */
    textColor?: string
    /** 스타일링을 위한 추가 CSS 클래스 */
    cssClass?: string
    /** 베테랑 배지의 경우 시간 임계값(밀리초) */
    veteranUserThresholdMillis?: number
    /** 이 배지가 사용자의 댓글에 표시되는지 여부 */
    displayedOnComments: boolean
    /** 배지의 표시 순서 */
    order?: number
}
[inline-code-end]
---
`UserBadgeProgress`는 FastComments 시스템에서 사용자가 다양한 배지를 획득하기 위한 진행 상황을 나타내는 객체입니다.

이 추적은 사용자의 활동 및 커뮤니티 참여를 기반으로 언제 자동으로 배지를 수여할지 결정하는 데 도움이 됩니다.

The structure for the `UserBadgeProgress` object is as follows:

[inline-code-attrs-start title = 'UserBadgeProgress 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface UserBadgeProgress {
    /** 이 진행 기록의 고유 식별자 */
    id: string
    /** 이 진행 기록이 속한 테넌트의 ID */
    tenantId: string
    /** 이 진행 기록이 추적하는 사용자의 ID */
    userId: string
    /** 시스템에서 사용자의 첫 댓글 ID */
    firstCommentId?: string
    /** 사용자의 첫 댓글 날짜(에포크 이후 밀리초) */
    firstCommentDate?: number
    /** 사용자 활동을 기반으로 자동 계산된 신뢰도 */
    autoTrustFactor?: number
    /** 관리자가 수동으로 설정한 신뢰도 */
    manualTrustFactor?: number
    /** 여러 지표를 포함한 상세 진행 객체, 키는 BadgeType 열거형과 일치함 */
    progress: {
        /** 0: CommentCount - 사용자가 작성한 댓글 수 */
        '0'?: number
        /** 1: CommentUpVotes - 사용자가 받은 추천(업보트) 수 */
        '1'?: number
        /** 2: CommentReplies - 사용자가 작성한 답글 수 */
        '2'?: number
        /** 3: CommentsPinned - 사용자가 고정한 댓글 수 */
        '3'?: number
        /** 4: Veteran - 사용자 계정 생성 후 경과 시간 */
        '4'?: number
        /** 5: NightOwl - 사용자가 야간 시간대에 게시한 횟수 */
        '5'?: number
        /** 6: FastReplyTime - 평균 답글 시간(밀리초) */
        '6'?: number
        /** 7: ModeratorCommentsDeleted - 모더레이터 배지의 경우 삭제된 댓글 수 */
        '7'?: number
        /** 8: ModeratorCommentsApproved - 모더레이터 배지의 경우 승인된 댓글 수 */
        '8'?: number
        /** 9: ModeratorCommentsUnapproved - 모더레이터 배지의 경우 미승인된 댓글 수 */
        '9'?: number
        /** 10: ModeratorCommentsReviewed - 모더레이터 배지의 경우 검토된 댓글 수 */
        '10'?: number
        /** 11: ModeratorCommentsMarkedSpam - 모더레이터 배지의 경우 스팸으로 표시된 댓글 수 */
        '11'?: number
        /** 12: ModeratorCommentsMarkedNotSpam - 모더레이터 배지의 경우 스팸 아님으로 표시된 댓글 수 */
        '12'?: number
        /** 13: RepliedToSpecificPage - 각 페이지에 대한 답글 수 */
        '13'?: Record<string, number>
    }
}
[inline-code-end]
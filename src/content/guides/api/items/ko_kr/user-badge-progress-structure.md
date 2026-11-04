`UserBadgeProgress`는 FastComments 시스템에서 사용자가 다양한 배지를 획득하기 위한 진행 상황을 나타내는 객체입니다.

이 추적은 커뮤니티에서 사용자의 활동과 참여를 기반으로 사용자가 자동 배지를 받아야 하는 시점을 결정하는 데 도움이 됩니다.

`UserBadgeProgress` 객체의 구조는 다음과 같습니다:

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
    /** 사용자의 첫 댓글 날짜 (epoch 이후 밀리초) */
    firstCommentDate?: number
    /** 사용자 활동을 기반으로 자동 계산된 신뢰 계수 */
    autoTrustFactor?: number
    /** 관리자가 수동으로 설정한 신뢰 계수 */
    manualTrustFactor?: number
    /** 다양한 지표가 포함된 상세 진행 객체, 키는 BadgeType 열거형과 일치 */
    progress: {
        /** 0: CommentCount - 사용자가 작성한 댓글 수 */
        '0'?: number
        /** 1: CommentUpVotes - 사용자가 받은 추천 수 */
        '1'?: number
        /** 2: CommentReplies - 사용자가 작성한 답글 수 */
        '2'?: number
        /** 3: CommentsPinned - 사용자가 보유한 고정된 댓글 수 */
        '3'?: number
        /** 4: Veteran - 사용자 계정의 나이 */
        '4'?: number
        /** 5: NightOwl - 사용자가 야간 시간대에 게시한 횟수 */
        '5'?: number
        /** 6: FastReplyTime - 평균 답변 시간 (밀리초) */
        '6'?: number
        /** 7: ModeratorCommentsDeleted - 모더레이터 배지의 경우, 삭제된 댓글 수 */
        '7'?: number
        /** 8: ModeratorCommentsApproved - 모더레이터 배지의 경우, 승인된 댓글 수 */
        '8'?: number
        /** 9: ModeratorCommentsUnapproved - 모더레이터 배지의 경우, 미승인 댓글 수 */
        '9'?: number
        /** 10: ModeratorCommentsReviewed - 모더레이터 배지의 경우, 검토한 댓글 수 */
        '10'?: number
        /** 11: ModeratorCommentsMarkedSpam - 모더레이터 배지의 경우, 스팸으로 표시한 댓글 수 */
        '11'?: number
        /** 12: ModeratorCommentsMarkedNotSpam - 모더레이터 배지의 경우, 스팸이 아니라고 표시한 댓글 수 */
        '12'?: number
        /** 13: RepliedToSpecificPage - 각 페이지별 답글 수 */
        '13'?: Record<string, number>
    }
}
[inline-code-end]
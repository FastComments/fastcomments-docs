`UserBadge` is an object that represents a badge assigned to a user in the FastComments system.

Badges can be assigned to users automatically based on their activity (such as comment count, reply time, veteran status) or manually by site administrators.

The structure for the `UserBadge` object is as follows:

[inline-code-attrs-start title = 'UserBadge Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface UserBadge {
    /** Unique identifier for this user badge assignment */
    id: string
    /** ID of the user this badge is assigned to */
    userId: string
    /** ID of the badge definition from the tenant's badge catalog */
    badgeId: string
    /** ID of the tenant that created/assigned this badge */
    fromTenantId: string
    /** When this badge was created (milliseconds since epoch) */
    createdAt?: number
    /** When this badge was received by the user (milliseconds since epoch) */
    receivedAt?: number
    /** 
     * The badge type: 
     * 0=CommentCount, 1=CommentUpVotes, 2=CommentReplies, 3=CommentsPinned, 
     * 4=Veteran, 5=NightOwl, 6=FastReplyTime, 7=ModeratorCommentsDeleted,
     * 8=ModeratorCommentsApproved, 9=ModeratorCommentsUnapproved, 10=ModeratorCommentsReviewed,
     * 11=ModeratorCommentsMarkedSpam, 12=ModeratorCommentsMarkedNotSpam, 13=RepliedToSpecificPage,
     * 14=Manual
     */
    type: number
    /** For threshold-based badges, the threshold value */
    threshold?: number
    /** The name/label of the badge */
    name?: string
    /** Detailed description of the badge */
    description?: string
    /** The text shown on the badge */
    displayLabel?: string
    /** URL to an image shown on the badge */
    displaySrc?: string
    /** Background color for the badge (hex code) */
    backgroundColor?: string
    /** Border color for the badge (hex code) */
    borderColor?: string
    /** Text color for the badge (hex code) */
    textColor?: string
    /** Additional CSS class for styling */
    cssClass?: string
    /** For veteran badges, the time threshold in milliseconds */
    veteranUserThresholdMillis?: number
    /** Whether this badge is displayed on the user's comments */
    displayedOnComments: boolean
    /** The display order of the badge */
    order?: number
}
[inline-code-end]
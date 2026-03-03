---
`UserBadge` הוא אובייקט שמייצג תג שמוקצה למשתמש במערכת FastComments.

תגים יכולים להיות מוקצים למשתמשים באופן אוטומטי על בסיס הפעילות שלהם (כגון מספר תגובות, זמן תגובה, סטטוס וותק) או באופן ידני על ידי מנהלי האתר.

המבנה של האובייקט `UserBadge` הוא כדלקמן:

[inline-code-attrs-start title = 'מבנה UserBadge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface UserBadge {
    /** מזהה ייחודי להקצאת תג המשתמש הזו */
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
     * סוג התג: 
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
    /** If set, this badge is only displayed on the page with the matching urlId. Null for global badges. */
    urlId?: string | null
}
[inline-code-end]
---
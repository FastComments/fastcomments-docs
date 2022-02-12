A `Moderator` object represents configuration for a moderator.

There are three types of moderators:

1. Administrator users that have the `isCommentModeratorAdmin` flag.
2. SSO Users with the `isCommentModeratorAdmin` flag.
3. Regular commenters, or FastComments.com users, that are invited as Moderators.

The `Moderator` structure is used to represent the Moderation State of use case `3`.

If you want to invite a user to be a moderator, via the API, use the `Moderator` API by creating a `Moderator` and `inviting` them.

If the user does not have a FastComments.com account, the invite email will help them get setup. If they already have an account, they will
be given moderation access to your tenant and the `Moderator` object's `userId` will be updated to point to their user. You will not have API
access to their user, as in this case it belongs to themselves and managed by FastComments.com.

If you require complete management of the user's account, we recommend either using SSO, or adding them as a [Tenant User](https://fastcomments.com/auth/my-account/users) and
then adding a `Moderator` object to track their stats.

The `Moderator` structure can be used as a stat tracking mechanism for use cases `1` and `2`. After creating the user, add a `Moderator`
object with their `userId` defined and their stats will be tracked on the [Comment Moderators Page](https://fastcomments.com/auth/my-account/moderate-comments/moderators).

The structure for the `Moderator` object is as follows:

[inline-code-attrs-start title = 'Moderator Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Moderator {
    name: string
    email: string
    tenantId: string
    userId?: string|null
    acceptedInvite?: boolean
    markReviewedCount?: number
    deletedCount?: number
    markedSpamCount?: number
    approvedCount?: number
    editedCount?: number
    bannedCount?: number
    createdAt: string
}
[inline-code-end]

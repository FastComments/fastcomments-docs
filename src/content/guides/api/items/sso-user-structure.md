FastComments provides an easy to use SSO solution. Updating a user's information with the HMAC-based integration is
as simple as having the user load the page with an updated payload.

However, it may be desirable to manage a user outside that flow, to improve consistency of your application.

The SSO User API provides a way to CRUD objects that we call SSOUsers. These objects are different from regular Users and
kept separate for type safety.

The structure for the SSOUser object is as follows:

[inline-code-attrs-start title = 'SSOUser Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    id: string;
    username: string;
    websiteUrl: string;
    email: string;
    signUpDate: number;
    createdFromUrlId: string;
    loginCount: number;
    avatarSrc: string;
    optedInNotifications: boolean;
    displayLabel?: string;
    displayName?: string;
    isAccountOwner?: boolean;
    isAdminAdmin?: boolean;
    isCommentModeratorAdmin?: boolean;
    /** If null, Access Control will not be applied to the user. If an empty list, this user will not be able to see any pages or @mention other users. **/
    groupIds?: string[] | null;
}
[inline-code-end]

### Access Control

Users can be broken into groups. This is what the `groupIds` field is for, and is optional.

### @Mentions

By default `@mentions` will use `username` to search for other sso users when the `@` character is typed. If `displayName` is used, then results matching
`username` will be ignored when there is a match for `displayName`, and the `@mention` search results will use `displayName`.

The `@mention` tag itself will always be displayed with the `username` to ensure it is unique.

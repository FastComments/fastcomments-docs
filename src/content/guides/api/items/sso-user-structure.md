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
    displayLabel: string;
    isAccountOwner?: boolean;
    isAdminAdmin?: boolean;
    isCommentModeratorAdmin?: boolean;
}
[inline-code-end]

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

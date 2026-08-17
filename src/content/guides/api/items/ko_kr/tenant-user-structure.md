The `TenantUser`는 특정 테넌트에 의해 관리되는 `User`를 정의합니다. 해당 계정은 연관된 테넌트가 완전히 제어하며, 계정은 [UI](https://fastcomments.com/auth/my-account/users) 또는 API를 통해 업데이트하거나 삭제할 수 있습니다.

테넌트 사용자는 `Tenant`에 대한 모든 권한과 접근을 가진 관리자일 수 있거나, 댓글을 관리하고 API 키에 접근하는 등 특정 권한만 제한적으로 가질 수 있습니다.

`TenantUser` 객체의 구조는 다음과 같습니다:

[inline-code-attrs-start title = 'TenantUser 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
/** 알림을 위한 것입니다. **/
export enum UserDigestEmailFrequency {
    Disabled = -1,
    Daily = 0,
    Weekly = 1,
    Monthly = 2
}

export interface TenantUser {
    id: string
    tenantId: string
    username: string
    /** 예를 들어, 댓글 작성자의 블로그에 대한 링크. **/
    websiteUrl?: string
    email: string
    signUpDate: number
    createdFromUrlId: string
    createdFromTenantId: string
    verified: boolean
    loginCount: number
    optedInNotifications: boolean
    optedInTenantNotifications: boolean
    hideAccountCode: boolean
    avatarSrc?: string
    /** 사용자가 댓글 작성자로부터 도움 요청을 받습니까? **/
    isHelpRequestAdmin: boolean
    isAccountOwner: boolean
    isAdminAdmin: boolean
    isBillingAdmin: boolean
    isAnalyticsAdmin: boolean
    isCustomizationAdmin: boolean
    isManageDataAdmin: boolean
    isCommentModeratorAdmin: boolean
    isAPIAdmin: boolean
    moderatorIds: string[]
    locale: FastCommentsLocale
    digestEmailFrequency: UserDigestEmailFrequency
    lastLoginDate: string
    displayLabel?: string
    karma?: number
}
[inline-code-end]
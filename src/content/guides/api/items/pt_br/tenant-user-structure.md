O `TenantUser` define um `User` que é gerenciado por um tenant específico. A conta deles está sob completo controle do tenant ao qual estão associados, e sua conta pode ser atualizada ou excluída via a [UI](https://fastcomments.com/auth/my-account/users) ou API.

Os usuários do tenant podem ser administradores com todas as permissões e acesso ao `Tenant`, ou podem ser limitados a permissões específicas para moderar comentários, acessar chaves de API, etc.

A estrutura para o objeto `TenantUser` é a seguinte:

[inline-code-attrs-start title = 'Estrutura do TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
/** Isso é para notificações. **/
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
    /** Um link para o blog do comentarista, por exemplo. **/
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
    /** O usuário recebe pedidos de ajuda de comentaristas? **/
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
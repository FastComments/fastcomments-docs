Et `AuditLog`-objekt oprettes, når en handling udføres i FastComments.

Bemærk, at AuditLogs har en TTL på et år efter oprettelse. De kan ikke gendannes, når de er udløbet.

Strukturen for `AuditLog`-objektet er som følger:

[inline-code-attrs-start title = 'AuditLog Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLog {
    id: string
    tenantId: string
    typeId: number
    /** You can cast the extra data on the object based on the typeId. See the Types enum below. **/
    data: Record<string, string | number>
    createdAt: string // date string
    createdBy: string // user id
    createdByName: string
}
[inline-code-end]

### Typer

Her er de mulige typer af audit log-poster sammen med `data`, du kan forvente fra hver type:

[inline-code-attrs-start title = 'AuditLog Typer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum AuditLogType {
    APICreds = 1, // { method: 'viewed' | 'created' | 'removed', apiKeyIdStart: string }
    AddModerator = 2, // { email: string, moderatorId: string }
    RemoveModerator = 3, // { email: string, moderatorId: string }
    AddTenantUser = 4, // { email: string, tenantUserId: string }
    RemoveTenantUser = 5, // { email: string, tenantUserId: string }
    UpdateBilling = 6,
    UpdatePaymentMethod = 7,
    NewDomain = 8, // { domain: string }
    RemoveDomain = 9, // { domain: string }
    NewUnverifiedComment = 10, // { urlId: string, commentId: string }
    NewComment = 11, // { urlId: string, commentId: string }
    EditComment = 12, // { urlId: string, commentId: string }
    RemoveComment = 13, // { urlId: string, commentId: string }
    AnonymizeComment = 14, // { urlId: string, commentId: string }
    RestoreComment = 15, // { urlId: string, commentId: string }
    MarkCommentSpam = 16, // { urlId: string, commentId: string }
    MarkCommentNotSpam = 17, // { urlId: string, commentId: string }
    ApproveComment = 18, // { urlId: string, commentId: string }
    FlagComment = 19, // { urlId: string, commentId: string }
    UnFlagComment = 20, // { urlId: string, commentId: string }
    PinComment = 21, // { urlId: string, commentId: string }
    UnPinComment = 22, // { urlId: string, commentId: string }
    BlockUser = 23, // { userId: string }
    UnBlockUser = 24, // { userId: string }
    WhitelistUser = 25, // { userId: string }
    UnWhitelistUser = 26, // { userId: string }
    ShadowBanUser = 27, // { userId: string }
    UnShadowBanUser = 28, // { userId: string }
    AddWordToCustomProfanityList = 29, // { word: string }
    RemoveWordFromCustomProfanityList = 30, // { word: string }
    Import = 31, // { domain: string }
    Export = 32,
    ExportByPage = 33, // { urlId: string }
    RemoveAllForPage = 34, // { urlId: string }
    Closeout = 35, // { newTenantId: string }
    UpdateModerator = 36, // { email: string, moderatorId: string }
    RemoveAllForUser = 37, // { userId: string }
    AnonymizeUser = 38, // { userId: string }
    UpdateSSOUser = 39, // { ssoUserId: string }
    RemoveSSOUser = 40, // { ssoUserId: string }
    AddDKIM = 41, // { domain: string }
    RemoveDKIM = 42, // { domain: string }
    AddHashTag = 43, // { tag: string, hashTagId: string }
    RemoveHashTag = 44, // { tag: string, hashTagId: string }
    UpdateHashTag = 45, // { tag: string, hashTagId: string }
    UpdateTenantUser = 46, // { email: string, tenantUserId: string }
}
[inline-code-end]

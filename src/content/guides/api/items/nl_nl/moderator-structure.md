Een `Moderator`-object vertegenwoordigt de configuratie voor een moderator.

Er zijn drie typen moderators:

1. Beheerdersgebruikers die de `isCommentModeratorAdmin` flag hebben.
2. SSO-gebruikers met de `isCommentModeratorAdmin` flag.
3. Gewone commentatoren, of FastComments.com-gebruikers, die worden uitgenodigd als Moderators.

De `Moderator`-structuur wordt gebruikt om de moderatiestatus van use case `3` weer te geven.

Als u een gebruiker via de API wilt uitnodigen om moderator te worden, gebruik dan de `Moderator` API door een `Moderator` aan te maken en ze te `inviting`.

Als de gebruiker geen FastComments.com-account heeft, helpt de uitnodigings-e-mail hen bij het opzetten. Als ze al een account hebben, krijgen ze moderatietoegang tot uw tenant en zal het `userId` van het `Moderator`-object worden bijgewerkt om naar hun gebruiker te verwijzen. U zult geen API-toegang tot hun gebruiker hebben, omdat deze in dat geval van henzelf is en wordt beheerd door FastComments.com.

Als u volledige beheer over het account van de gebruiker nodig heeft, raden we aan ofwel SSO te gebruiken, of ze toe te voegen als een [Tenant-gebruiker](https://fastcomments.com/auth/my-account/users) en vervolgens een `Moderator`-object toe te voegen om hun statistieken bij te houden.

De `Moderator`-structuur kan worden gebruikt als een mechanisme voor statistiekregistratie voor use cases `1` en `2`. Nadat u de gebruiker hebt aangemaakt, voegt u een `Moderator`-object toe met hun `userId` gedefinieerd en hun statistieken worden bijgehouden op de [Pagina voor comment-moderatoren](https://fastcomments.com/auth/my-account/moderate-comments/moderators).

De structuur van het `Moderator`-object is als volgt:

[inline-code-attrs-start title = 'Structuur van Moderator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    moderationGroupIds?: string[]|null
}
[inline-code-end]

---
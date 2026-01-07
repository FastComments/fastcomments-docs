Et `Moderator`-objekt repræsenterer konfiguration for en moderator.

Der er tre typer moderatorer:

1. Administratorbrugere, der har `isCommentModeratorAdmin`-flaget.
2. SSO-brugere med `isCommentModeratorAdmin`-flaget.
3. Almindelige kommentatorer eller FastComments.com-brugere, der er inviteret som Moderatorer.

`Moderator`-strukturen bruges til at repræsentere Moderationstilstanden for anvendelsestilfælde `3`.

Hvis du vil invitere en bruger til at være moderator via API'et, brug `Moderator` API'et ved at oprette en `Moderator` og `invitere` dem.

Hvis brugeren ikke har en FastComments.com-konto, vil invitations-e-mailen hjælpe dem med at blive sat op. Hvis de allerede har en konto, vil de
få moderationsadgang til din tenant, og `Moderator`-objektets `userId` vil blive opdateret til at pege på deres bruger. Du vil ikke have API-
adgang til deres bruger, da den i dette tilfælde tilhører dem selv og administreres af FastComments.com.

Hvis du kræver fuld styring af brugerens konto, anbefaler vi enten at bruge SSO eller tilføje dem som en [Tenant Bruger](https://fastcomments.com/auth/my-account/users) og
derefter tilføje et `Moderator`-objekt for at spore deres statistikker.

`Moderator`-strukturen kan bruges som en statistik-sporingsmekanisme for anvendelsestilfælde `1` og `2`. Efter oprettelse af brugeren, tilføj et `Moderator`-
objekt med deres `userId` defineret, og deres statistikker vil blive sporet på [Kommentarmoderatorer-siden](https://fastcomments.com/auth/my-account/moderate-comments/moderators).

Strukturen for `Moderator`-objektet er som følger:

[inline-code-attrs-start title = 'Moderator Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

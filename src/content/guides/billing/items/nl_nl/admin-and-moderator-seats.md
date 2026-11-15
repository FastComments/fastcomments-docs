Op Flex worden mensen die uw account kunnen beheren of modereren gefactureerd als maandelijkse seats. Dit is het onderdeel van een Flex-rekening dat mensen het vaakst verrast, meestal omdat seats zijn aangemaakt zonder dat iemand het in de gaten had.

### Wat Telt Als Een Seat

- **Admins**: gebruikers op de `Users`-pagina van uw account met een administratieve permissie (facturering, aanpassing, analytics, moderatie, enzovoort). De eerste admin, normaal gesproken de accounteigenaar, is gratis; elke extra admin is een seat.
- **Moderators**: mensen op de `Moderators`-pagina die hun uitnodiging hebben geaccepteerd. Een uitnodiging in behandeling wordt niet gefactureerd. Een gebruiker wiens enige permissie het modereren van reacties is, wordt gefactureerd als moderator, niet als admin.
- **SSO Admins** en **SSO Moderators**: SSO-gebruikers waarmee uw site inlogt met `isAccountOwner`, `isAdminAdmin`, of `isCommentModeratorAdmin` ingesteld in de SSO-payload. Zij worden tegen dezelfde tarieven gefactureerd als gewone admins en moderators. Zie [SSO User Structure](/guide-api.html#sso-user-structure).

Seats worden gededupliceerd op e-mailadres, dus één persoon die zowel admin als moderator is, of die zowel een SSO-gebruiker als een dashboard-login met hetzelfde e-mailadres heeft, wordt één keer gefactureerd, tegen het hogere tarief. Iemand met twee verschillende e-mailadressen is twee seats.

Op accounts die worden gefactureerd op basis van maandelijkse actieve gebruikers wordt een SSO-moderator die die maand niet heeft ingelogd, niet voor die maand gefactureerd.

### Seats Die U Niet Heeft Toegevoegd

Twee functies maken automatisch seats aan:

- **SSO admin sync.** Wanneer een SSO-gebruiker inlogt met een e-mailadres dat overeenkomt met een van uw dashboard-gebruikers, worden de admin- of moderator-permissies van die gebruiker gekopieerd naar de SSO-gebruiker. Met WordPress SSO betekent dit dat elke WordPress-beheerder een FastComments-admin of -moderator kan worden. Schakel dit uit onder `Customize` met de optie `Disable Automatic Admin Migration` in uw aanpassingsregel.
- **Verschillende e-mails voor dezelfde persoon.** Als uw dashboard-login één e-mailadres gebruikt en uw SSO-gebruiker een ander, zijn dit twee seats. Door hetzelfde e-mailadres voor beide te gebruiken, worden ze samengevoegd.

### Seats Controleren En Verwijderen

De pagina [Billing Analytics](https://fastcomments.com/auth/my-account/analytics/billing) toont uw aantallen `Moderators`, `SSO Admins` en `SSO Moderators`, en het voorbeeld van de volgende factuur toont wat deze kosten. Verwijder mensen die u niet nodig heeft van de `Users`- en `Moderators`-pagina's, of verwijder de admin-vlaggen uit uw SSO-payload. Seats worden geteld wanneer de factuur aan het einde van de maand wordt aangemaakt, dus een seat die daarvoor wordt verwijderd, wordt niet voor die maand gefactureerd.

Zie [Adding Moderators](/guide-users.html#adding-moderators) en [Updating Administrator Permissions](/guide-users.html#updating-administrator-permissions) voor hoe u beide beheert.
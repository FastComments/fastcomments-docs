På Flex faktureres personer, der kan administrere eller moderere din konto, som månedlige pladser. Dette er den del af en Flex-regning, der oftest overrasker folk, normalt fordi pladser blev oprettet uden at nogen bemærkede det.

### Hvad Tæller Som En Plads

- **Admins**: brugere på din kontos `Users`-side med enhver administrativ tilladelse (fakturering, tilpasning, analyse, moderation osv.). Den første admin, normalt kontoens ejer, er gratis; hver ekstra admin er en plads.
- **Moderators**: personer på `Moderators`-siden, der har accepteret deres invitation. En afventende invitation faktureres ikke. En bruger, hvis eneste tilladelse er kommentarmoderation, faktureres som moderator, ikke som admin.
- **SSO Admins** og **SSO Moderators**: SSO-brugere, som dit site logger ind med `isAccountOwner`, `isAdminAdmin` eller `isCommentModeratorAdmin` sat i SSO-payloaden. De faktureres til samme satser som almindelige admins og moderatorer. Se [SSO User Structure](/guide-api.html#sso-user-structure).

Pladser deduplikeres via e-mailadresse, så én person, der både er admin og moderator, eller som har både en SSO-bruger og et dashboard-login med samme e-mail, faktureres én gang, til den højere sats. En person med to forskellige e-mailadresser er to pladser.

På konti, der faktureres efter månedligt aktive brugere, faktureres en SSO-moderator, der ikke loggede ind i løbet af måneden, ikke for den måned.

### Pladser Du Ikke Har Tilføjet

To funktioner opretter pladser automatisk:

- **SSO admin-synkronisering.** Når en SSO-bruger logger ind med en e-mail, der matcher en af dine dashboard-brugere, kopieres brugerens admin- eller moderator-tilladelser til SSO-brugeren. Med WordPress SSO betyder det, at enhver WordPress-administrator kan blive FastComments-admin eller -moderator. Slå det fra under `Customize` med indstillingen `Disable Automatic Admin Migration` på din tilpasningsregel.
- **Forskellige e-mails for samme person.** Hvis dit dashboard-login bruger én e-mail, og din SSO-bruger en anden, er det to pladser. Brug af samme e-mail på begge fletter dem sammen.

### Tjek Og Fjern Pladser

Siden [Billing Analytics](https://fastcomments.com/auth/my-account/analytics/billing) viser dine antal `Moderators`, `SSO Admins` og `SSO Moderators`, og forhåndsvisningen af den næste faktura viser, hvad de koster. Fjern personer, du ikke har brug for, fra `Users`- og `Moderators`-siderne, eller fjern admin-flagene fra din SSO-payload. Pladser tælles, når fakturaen oprettes ved månedens udgang, så en plads, der fjernes inden da, faktureres ikke for den måned.

Se [Adding Moderators](/guide-users.html#adding-moderators) og [Updating Administrator Permissions](/guide-users.html#updating-administrator-permissions) for, hvordan du administrerer begge.
Alle målinger, som du kan blive rate limited eller faktureret for, rapporteres på [Billing Analytics Page](https://fastcomments.com/auth/my-account/analytics/billing).

Nogle eksempler på målinger, der spores:

- Page Loads (for den aktuelle måned)
- API Credits (for den aktuelle måned)
- New Comments Stored (for den aktuelle måned)
- Tenant Users
- SSO Users (almindelige SSO-brugere uden admin/moderator-rettigheder)
- SSO Admins (SSO-brugere med admin-flaggene `isAccountOwner` eller `isAdminAdmin` - faktureres separat fra almindelige SSO-brugere)
- SSO Moderators (SSO-brugere med flagget `isCommentModeratorAdmin` - faktureres separat fra almindelige SSO-brugere)
- Moderators
- Domains

For `Creators`- og `Pro`-planerne fungerer disse målinger blot som en vejledning, så du kan se, om du er tæt på eller har nået grænserne for din konto.

For `Flex`-planen viser denne side en omkostningsopdeling af den aktuelle forbrug for den **næste** faktura. Det gør det nemt at forstå, hvordan du
bliver faktureret, før det sker. Du kan se tidligere fakturaer under [Billing History](https://fastcomments.com/auth/my-account/billing-history). Hvordan hver linje tælles, og hvornår fakturan opkræves, er forklaret i [Billing guide](/guide-billing.html#how-flex-usage-is-measured).

Målingerne `Page Loads`, `API Credits` og `New Comments Stored` kan være op til ét minut forældede. De andre målinger er i realtid.

Disse oplysninger gemmes månedligt i ét år, eller indtil du sletter din tenant, af historiske årsager, men brugergrænsefladen viser kun den aktuelle periode.
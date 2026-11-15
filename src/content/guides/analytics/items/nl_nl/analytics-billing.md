Alle metrieken waarvoor je mogelijk kunt worden beperkt (rate limited) of gefactureerd, worden gerapporteerd op de [Billing Analytics Page](https://fastcomments.com/auth/my-account/analytics/billing).

Enkele voorbeeldmetrieken die worden bijgehouden:

- Page Loads (voor de huidige maand)
- API Credits (voor de huidige maand)
- New Comments Stored (voor de huidige maand)
- Tenant Users
- SSO Users (gewone SSO-gebruikers zonder admin/moderator-rechten)
- SSO Admins (SSO-gebruikers met de vlaggen `isAccountOwner` of `isAdminAdmin` - apart gefactureerd van gewone SSO-gebruikers)
- SSO Moderators (SSO-gebruikers met de vlag `isCommentModeratorAdmin` - apart gefactureerd van gewone SSO-gebruikers)
- Moderators
- Domains

Voor de `Creators`- en `Pro`-plannen dienen deze metrieken slechts als richtlijn om te weten of je dicht bij de limieten van je account bent of deze hebt bereikt.

Voor het `Flex`-plan toont deze pagina een kostenoverzicht van het huidige gebruik voor de **volgende** factuur. Zo is het eenvoudig om te begrijpen hoe je wordt gefactureerd voordat het zover is. Eerdere facturen kun je bekijken in [Billing History](https://fastcomments.com/auth/my-account/billing-history). Hoe elke regel wordt geteld en wanneer de factuur wordt afgeschreven, wordt uitgelegd in de [Billing guide](/guide-billing.html#how-flex-usage-is-measured).

De metrieken `Page Loads`, `API Credits` en `New Comments Stored` kunnen tot één minuut verouderd zijn. De overige metrieken zijn realtime.

Deze informatie wordt maandelijks opgeslagen gedurende één jaar, of totdat je je tenant verwijdert, voor historische doeleinden, maar de UI toont alleen de huidige periode.
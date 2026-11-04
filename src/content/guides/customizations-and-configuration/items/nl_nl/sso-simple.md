[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

Met Simple SSO kunnen we de commentaarwidget voorzien van informatie over de gebruiker, zodat ze hun gebruikersnaam of e‑mail niet hoeven in te voeren om te reageren.

We kunnen Simple SSO als volgt configureren:

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], pageBadgeIds: ['badge-id-3'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]; title = 'Simple SSO'; code-example-end]

De gebruiker zal ingelogd zijn en er wordt achter de schermen een SSO‑gebruiker aangemaakt. De gebruiker zal `createdFromSimpleSSO` op `true` hebben als deze via de API wordt opgehaald.

Opmerkingen:

- E‑mail is de unieke identifier voor Simple SSO.
- Het verstrekken van een e‑mail met Simple SSO is niet vereist, maar standaard worden hun reacties weergegeven als "Ongeverifieerd". <b>Als er geen e‑mail wordt opgegeven, kan de gebruiker niet volledig worden geauthenticeerd.</b>
- **NIEUW** Sinds jan 2022: Gebruikersnamen hoeven niet uniek te zijn op fastcomments.com
- Simple SSO kan automatisch SSO‑gebruikers aanmaken en bijwerken, als er een e‑mail wordt opgegeven en de gebruiker niet oorspronkelijk is aangemaakt via Secure SSO. Als een Simple SSO‑payload een gebruiker benoemt die is aangemaakt door Secure SSO of de API, blijft die gebruiker onaangeroerd en krijgt de bezoeker een aparte gebruiker.
- Je kunt badges voor de gebruiker opgeven met de eigenschap `badgeConfig`. De array `badgeIds` bevat de ID's van globale badges die aan de gebruiker worden gekoppeld. De array `pageBadgeIds` bevat badge‑ID's die zijn beperkt tot de huidige pagina (`urlId`) — deze badges worden alleen weergegeven op de pagina waaraan ze zijn toegewezen. Als `override` op `true` staat, vervangt deze de reeds weergegeven badges (globale en paginabeperkte badges worden onafhankelijk overschreven); als `false` worden ze toegevoegd aan de bestaande badges.

#### Simple SSO-instellingen

Simple SSO is altijd beschikbaar. Twee instellingen bepalen hoeveel FastComments erop vertrouwt. Je kunt ze vinden door te zoeken naar SSO in een <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">aanpassingsregel</a>. Beide staan standaard uit, en ze gelden ongeacht of "Require SSO To View Comments" is ingeschakeld.

Accounts die al Simple SSO gebruikten toen deze instellingen werden toegevoegd, hebben beide ingeschakeld, dus er is niets voor hen veranderd.

Alleen de aanpassingsregel die het beste bij een pagina past, wordt toegepast, dus als je meer dan één regel hebt, stel deze op elk van hen in. Een nieuwe regel begint met deze ingeschakeld als een van je andere regels ze ingeschakeld heeft.

##### Automatisch verifiëren van reacties via Simple SSO

Wanneer dit is ingeschakeld, worden reacties van Simple SSO‑gebruikers direct geverifieerd.

Wanneer dit is uitgeschakeld, worden reacties van Simple SSO‑gebruikers weergegeven als "Ongeverifieerd" totdat de gebruiker op een link klikt die we per e‑mail naar het adres in de Simple SSO‑payload sturen. Als je alleen automatisch geverifieerde reacties goedkeurt, wachten ze ook op die link. Als er geen e‑mail in de payload zit, blijft de reactie ongeverifieerd.

Deze instelling heeft alleen invloed op reacties. Stemmen van Simple SSO‑gebruikers worden altijd direct geteld.

##### Toestaan van Simple SSO-beheerders

Wanneer dit is ingeschakeld, kan een Simple SSO‑gebruiker een beheerder of moderator van je account zijn:

- Een Simple SSO‑gebruiker die admin‑ of moderatorrechten heeft, behoudt deze.
- Een Simple SSO‑gebruiker met dezelfde e‑mail als een van je beheerders of moderators krijgt hun rechten.

Wanneer dit is uitgeschakeld, verleent Simple SSO deze rechten niet. Een Simple SSO‑payload die een gebruiker benoemt die deze rechten heeft, krijgt een aparte gebruiker zonder rechten, en de oorspronkelijke gebruiker blijft onaangeroerd.

Simple SSO‑payloads zijn niet ondertekend, dus wanneer dit is ingeschakeld, kan iedereen die de e‑mail of gebruikers‑ID van een beheerder kent, zich voordoen als die persoon. Als dat van belang is voor je site, laat dit dan uitgeschakeld en gebruik Secure SSO voor je beheerders.

Gebruikers die zijn aangemaakt door Secure SSO of de API kunnen niet worden bereikt door Simple SSO, ongeacht of dit is ingeschakeld.

---
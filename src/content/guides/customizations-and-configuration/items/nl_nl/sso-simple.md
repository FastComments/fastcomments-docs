[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

Met Simple SSO kunnen we de reactie-widget voorzien van informatie over de gebruiker, zodat zij hun gebruikersnaam of e-mailadres niet hoeven in te vullen om te reageren.

We kunnen Simple SSO als volgt configureren:

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18]; title = 'Eenvoudige SSO'; code-example-end]

De gebruiker wordt ingelogd en er wordt achter de schermen een SSO-gebruiker aangemaakt. De gebruiker zal `createdFromSimpleSSO` op `true` hebben staan als deze via de API is opgehaald.

Notes: 

- E-mail is de unieke identificatie voor Simple SSO.
- Het opgeven van een e-mail bij Simple SSO is niet verplicht, maar standaard worden hun reacties weergegeven als "Niet geverifieerd". <b>Als er geen e-mail wordt opgegeven, kan de gebruiker niet volledig worden geverifieerd.</b>
- **NEW** Sinds jan 2022: gebruikersnamen hoeven niet uniek te zijn over heel fastcomments.com
- Simple SSO kan automatisch SSO-gebruikers aanmaken en bijwerken als er een e-mailadres is opgegeven en de gebruiker niet oorspronkelijk via Secure SSO is aangemaakt.
- U kunt badges voor de gebruiker specificeren met de `badgeConfig` property. De `badgeIds` array bevat de IDs van badges die aan de gebruiker gekoppeld moeten worden. Als `override` is ingesteld op `true`, vervangt dit alle bestaande badges die bij reacties worden weergegeven; als `false` wordt het toegevoegd aan de bestaande badges.
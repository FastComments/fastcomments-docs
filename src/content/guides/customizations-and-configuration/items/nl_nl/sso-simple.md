[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

Met Simple SSO kunnen we de reactie-widget voorzien van informatie over de gebruiker, zodat ze hun gebruikersnaam of e-mailadres niet hoeven in te voeren om te reageren.

We kunnen Simple SSO als volgt configureren:

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], pageBadgeIds: ['badge-id-3'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]; title = 'Simple SSO'; code-example-end]

De gebruiker zal ingelogd zijn, en er wordt achter de schermen een SSO-gebruiker aangemaakt. De gebruiker zal `createdFromSimpleSSO` op `true` hebben staan als deze via de API is opgehaald.

Notes: 

- E-mailadres is de unieke identificator voor Simple SSO.
- Het opgeven van een e-mailadres bij Simple SSO is niet vereist; standaard worden hun opmerkingen echter weergegeven als "Unverified". <b>Als er geen e-mailadres wordt opgegeven, kan de gebruiker niet volledig worden geverifieerd.</b>
- **NEW** Sinds Jan 2022: Gebruikersnamen hoeven niet uniek te zijn binnen fastcomments.com
- Simple SSO kan automatisch SSO-gebruikers aanmaken en bijwerken als een e-mailadres is opgegeven en de gebruiker oorspronkelijk niet via Secure SSO is aangemaakt.
- Je kunt badges voor de gebruiker specificeren met de `badgeConfig` property. De `badgeIds` array bevat de IDs van globale badges die aan de gebruiker gekoppeld moeten worden. De `pageBadgeIds` array bevat badge IDs die zijn gebonden aan de huidige pagina (`urlId`) — deze badges worden alleen weergegeven op de pagina waar ze zijn toegewezen. Als `override` is ingesteld op `true`, zal het de bestaande weergegeven badges vervangen (globale en pagina-gescopeerde badges worden onafhankelijk overschreven); als `false` wordt gebruikt, worden ze toegevoegd aan bestaande badges.

---
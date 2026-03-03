[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

Con Simple SSO possiamo fornire al widget dei commenti informazioni sull'utente, così non dovrà inserire il proprio username o la propria email per commentare.

Possiamo configurare Simple SSO come segue:

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], pageBadgeIds: ['badge-id-3'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]; title = 'Simple SSO'; code-example-end]

L'utente verrà autenticato e verrà creato un SSO User in background. L'utente avrà `createdFromSimpleSSO` impostato su `true` se recuperato dall'API.

Notes: 

- L'email è l'identificatore univoco per Simple SSO.
- Fornire un'email con Simple SSO non è obbligatorio; tuttavia, per impostazione predefinita i loro commenti verranno mostrati come "Unverified". <b>Se non viene fornita un'email, l'utente non può essere completamente autenticato.</b>
- **NUOVO** Da gen 2022: i nomi utente non devono essere univoci su tutto fastcomments.com
- Simple SSO può creare e aggiornare automaticamente gli utenti SSO, se viene fornita un'email e l'utente non è stato creato originariamente tramite Secure SSO.
- Puoi specificare badge per l'utente con la proprietà `badgeConfig`. L'array `badgeIds` contiene gli ID dei badge globali da associare all'utente. L'array `pageBadgeIds` contiene gli ID dei badge limitati alla pagina corrente (`urlId`) — questi badge vengono visualizzati solo nella pagina in cui sono stati assegnati. Se `override` è impostato su `true`, sostituirà i badge già visualizzati (i badge globali e quelli a livello di pagina vengono sovrascritti indipendentemente); se `false`, verranno aggiunti ai badge esistenti.
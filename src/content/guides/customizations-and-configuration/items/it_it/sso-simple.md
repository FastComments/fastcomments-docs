[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

Con Simple SSO, possiamo fornire al widget dei commenti informazioni sull'utente in modo che non debba inserire il proprio nome utente o la propria email per commentare.

Possiamo configurare Simple SSO come segue:

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18]; title = 'Simple SSO'; code-example-end]

L'utente verrà autenticato e verrà creato un utente SSO in background. L'utente avrà `createdFromSimpleSSO` impostato su `true` se recuperato dall'API.

Notes: 

- L'email è l'identificatore univoco per Simple SSO.
- Fornire un'email con Simple SSO non è obbligatorio, tuttavia per impostazione predefinita i loro commenti verranno mostrati come "Non verificato". <b>Se non viene fornita un'email, l'utente non può essere completamente autenticato.</b>
- **NUOVO** Da gennaio 2022: i nomi utente non devono essere univoci su tutto fastcomments.com
- Simple SSO può creare e aggiornare automaticamente gli utenti SSO se viene fornita un'email e l'utente non è stato originariamente creato tramite Secure SSO.
- È possibile specificare badge per l'utente con la proprietà `badgeConfig`. L'array `badgeIds` contiene gli ID dei badge da associare all'utente. Se `override` è impostato su `true`, sostituirà tutti i badge esistenti visualizzati nei commenti; se `false` verrà aggiunto ai badge esistenti.
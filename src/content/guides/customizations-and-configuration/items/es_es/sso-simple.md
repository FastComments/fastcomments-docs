---
[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

Con Simple SSO, podemos proporcionar al widget de comentarios información sobre el usuario para que no tenga que introducir su nombre de usuario o correo electrónico para comentar.

Podemos configurar Simple SSO de la siguiente manera:

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18]; title = 'Simple SSO'; code-example-end]

El usuario iniciará sesión y se creará un Usuario SSO en segundo plano. El usuario tendrá `createdFromSimpleSSO` establecido en `true` si se recupera desde la API.

Notas: 

- El correo electrónico es el identificador único para Simple SSO.
- Proporcionar un correo electrónico con Simple SSO no es obligatorio; sin embargo, por defecto sus comentarios se mostrarán como "Sin verificar". <b>Si no se proporciona un correo electrónico, el usuario no puede ser completamente autenticado.</b>
- **NEW** Desde enero de 2022: los nombres de usuario no tienen que ser únicos en todo fastcomments.com
- Simple SSO puede crear y actualizar automáticamente usuarios SSO si se proporciona un correo electrónico y el usuario no fue creado originalmente mediante Secure SSO.
- Puede especificar insignias para el usuario con la propiedad `badgeConfig`. La matriz `badgeIds` contiene los IDs de las insignias que se asociarán con el usuario. Si `override` se establece en `true`, reemplazará todas las insignias existentes mostradas en los comentarios; si `false`, las añadirá a las existentes.

---
[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

Con Simple SSO, podemos proporcionar al widget de comentarios información sobre el usuario para que no tenga que ingresar su nombre de usuario o correo electrónico para comentar.

Podemos configurar Simple SSO de la siguiente manera:

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], pageBadgeIds: ['badge-id-3'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]; title = 'SSO Simple'; code-example-end]

El usuario iniciará sesión y se creará un Usuario SSO en segundo plano. El usuario tendrá `createdFromSimpleSSO` establecido en `true` si se obtiene de la API.

**Notas:** 

- El correo electrónico es el identificador único para Simple SSO.  
- Proporcionar un correo electrónico con Simple SSO no es obligatorio, sin embargo, por defecto sus comentarios se mostrarán como "Unverified". <b>Si no se proporciona correo electrónico, el usuario no puede autenticarse completamente.</b>  
- **NUEVO** Desde enero de 2022: Los nombres de usuario no tienen que ser únicos en todo fastcomments.com  
- Simple SSO puede crear y actualizar automáticamente usuarios SSO, si se proporciona un correo electrónico, y el usuario no fue creado originalmente a partir de Secure SSO. Si una carga Simple SSO nombra a un usuario creado por Secure SSO o la API, ese usuario queda sin cambios y el visitante obtiene un usuario separado.  
- Puedes especificar insignias para el usuario con la propiedad `badgeConfig`. La matriz `badgeIds` contiene los IDs de insignias globales que se asociarán con el usuario. La matriz `pageBadgeIds` contiene IDs de insignias limitadas a la página actual (`urlId`) — estas insignias solo se muestran en la página donde fueron asignadas. Si `override` se establece en `true`, reemplazará las insignias mostradas existentes (las globales y las limitadas a la página se sobrescriben de forma independiente); si es `false` se añadirán a las insignias existentes.

#### Configuración de Simple SSO

Simple SSO siempre está disponible. Dos configuraciones controlan cuánto confía FastComments en él. Puedes encontrarlas buscando SSO en una <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">regla de personalización</a>. Ambas están desactivadas por defecto, y se aplican tanto si "Require SSO To View Comments" está activado o no.

Las cuentas que ya estaban usando Simple SSO cuando se añadieron estas configuraciones tienen ambas activadas, por lo que no cambió nada para ellas.

Solo la regla de personalización que mejor coincida con una página se aplica, así que si tienes más de una regla, establece estas configuraciones en cada una de ellas. Una regla nueva comienza con ellas activadas si alguna de tus otras reglas las tiene activadas.

##### Verificar automáticamente los comentarios mediante Simple SSO

Cuando está activado, los comentarios de usuarios Simple SSO se verifican de inmediato.

Cuando está desactivado, los comentarios de usuarios Simple SSO se muestran como "Unverified" hasta que el usuario haga clic en un enlace que enviamos por correo a la dirección incluida en la carga Simple SSO. Si solo apruebas automáticamente los comentarios verificados, también esperarán ese enlace. Si no hay correo electrónico en la carga, el comentario permanece sin verificar.

Esta configuración solo afecta a los comentarios. Los votos de usuarios Simple SSO siempre se cuentan de inmediato.

##### Permitir administradores Simple SSO

Cuando está activado, un usuario Simple SSO puede ser administrador o moderador de tu cuenta:

- Un usuario Simple SSO que tiene permisos de administrador o moderador los conserva.  
- Un usuario Simple SSO con el mismo correo electrónico que uno de tus administradores o moderadores adquiere sus permisos.

Cuando está desactivado, Simple SSO no otorga estos permisos. Una carga Simple SSO que nombra a un usuario que los tiene genera un usuario separado sin permisos, y el usuario original queda sin cambios.

Las cargas Simple SSO no están firmadas, por lo que cuando está activado, cualquiera que conozca el correo electrónico o el ID de usuario de un administrador puede actuar como él. Si eso es importante para tu sitio, déjalo desactivado y usa Secure SSO para tus administradores.

Los usuarios creados por Secure SSO o la API no pueden ser alcanzados por Simple SSO, tanto si está activado como si no.
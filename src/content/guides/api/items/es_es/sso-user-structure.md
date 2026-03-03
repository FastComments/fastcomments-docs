FastComments proporciona una solución SSO fácil de usar. Actualizar la información de un usuario con la integración basada en HMAC es tan sencillo como hacer que el usuario cargue la página con una carga útil actualizada.

Sin embargo, puede ser deseable gestionar un usuario fuera de ese flujo, para mejorar la coherencia de su aplicación.

La API SSO User proporciona una forma de crear, leer, actualizar y eliminar (CRUD) objetos que llamamos SSOUsers. Estos objetos son diferentes de los Usuarios regulares y se mantienen separados por seguridad de tipos.

La estructura del objeto SSOUser es la siguiente:

[inline-code-attrs-start title = 'Estructura de SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    id: string
    username: string
    email?: string
    websiteUrl?: string
    signUpDate: number
    createdFromUrlId?: string
    loginCount?: number
    avatarSrc?: string
    optedInNotifications?: boolean
    optedInSubscriptionNotifications?: boolean
    displayLabel?: string
    displayName?: string
    isAccountOwner?: boolean // Permiso de administrador - los usuarios SSO con esta bandera se facturan como administradores SSO (separados de los usuarios SSO normales)
    isAdminAdmin?: boolean // Permiso de administrador - los usuarios SSO con esta bandera se facturan como administradores SSO (separados de los usuarios SSO normales)
    isCommentModeratorAdmin?: boolean // Permiso de moderador - los usuarios SSO con esta bandera se facturan como moderadores SSO (separados de los usuarios SSO normales)
    /** Si es null, no se aplicará Control de Acceso al usuario. Si es una lista vacía, este usuario no podrá ver ninguna página ni mencionar (@) a otros usuarios. **/
    groupIds?: string[] | null
    createdFromSimpleSSO?: boolean
    /** No permitir que otros usuarios vean la actividad de este usuario, incluidos los comentarios, en su perfil. El valor por defecto es true para proporcionar perfiles seguros por defecto. **/
    isProfileActivityPrivate?: boolean
    /** No permitir que otros usuarios dejen comentarios en el perfil del usuario, ni ver los comentarios de perfil existentes. Por defecto false. **/
    isProfileCommentsPrivate?: boolean
    /** No permitir que otros usuarios envíen mensajes directos a este usuario. Por defecto false. **/
    isProfileDMDisabled?: boolean
    karma?: number
    /** Configuración opcional para las insignias del usuario. **/
    badgeConfig?: {
        /** Array de IDs de insignias para asignar al usuario. Limitado a 30 insignias. Se respeta el orden. Estas son insignias globales visibles en todas las páginas. **/
        badgeIds: string[]
        /** Array de IDs de insignias con alcance en la página actual (urlId). Estas insignias solo se muestran en la página donde fueron asignadas. **/
        pageBadgeIds?: string[]
        /** Si true, reemplaza todas las insignias mostradas existentes por las proporcionadas. Las insignias globales y con alcance por página se sobrescriben de forma independiente. Si false, añade a las insignias existentes. **/
        override?: boolean
        /** Si true, actualiza las propiedades de visualización de las insignias desde la configuración del tenant. **/
        update?: boolean
    }
}
[inline-code-end]

### Facturación para usuarios SSO

Los usuarios SSO se facturan de manera diferente según sus banderas de permisos:

- **Usuarios SSO regulares**: Los usuarios sin permisos de administrador o moderador se facturan como usuarios SSO regulares
- **Administradores SSO**: Los usuarios con las banderas `isAccountOwner` o `isAdminAdmin` se facturan por separado como administradores SSO (misma tarifa que los administradores regulares del tenant)
- **Moderadores SSO**: Los usuarios con la bandera `isCommentModeratorAdmin` se facturan por separado como moderadores SSO (misma tarifa que los moderadores regulares)

**Importante**: Para evitar doble facturación, el sistema desduplicará automáticamente los usuarios SSO frente a los usuarios regulares del tenant y moderadores por dirección de correo electrónico. Si un usuario SSO tiene el mismo correo electrónico que un usuario regular del tenant o un moderador, no se le facturará dos veces.

### Control de Acceso

Los usuarios se pueden agrupar en grupos. Para ello sirve el campo `groupIds`, y es opcional.

### @Mentions

Por defecto `@mentions` usará `username` para buscar otros usuarios SSO cuando se escriba el carácter `@`. Si se usa `displayName`, entonces se ignorarán los resultados que coincidan con `username` cuando haya una coincidencia para `displayName`, y los resultados de búsqueda de `@mention` usarán `displayName`.

### Suscripciones

Con FastComments, los usuarios pueden suscribirse a una página haciendo clic en el icono de campana en el widget de comentarios y pulsando Suscribirse.

Con un usuario regular, les enviamos correos electrónicos de notificación según sus ajustes de notificación.

Con usuarios SSO, separamos esto por compatibilidad con versiones anteriores. Los usuarios solo recibirán estos correos electrónicos adicionales de notificación de suscripción si establece `optedInSubscriptionNotifications` en `true`.

### Insignias

Puedes asignar insignias a los usuarios SSO usando la propiedad `badgeConfig`. Las insignias son indicadores visuales que aparecen junto al nombre del usuario en los comentarios.

- `badgeIds` - Un array de IDs de insignias para asignar al usuario. Estas son insignias globales visibles en todas las páginas. Deben ser IDs de insignia válidos creados en su cuenta de FastComments. Limitado a 30 insignias.
- `pageBadgeIds` - Un array opcional de IDs de insignias con alcance en la página actual (`urlId`). Estas insignias solo se muestran en la página donde fueron asignadas. Diferentes páginas pueden tener distintas insignias con alcance por página para el mismo usuario.
- `override` - Si es true, todas las insignias mostradas existentes serán reemplazadas por las proporcionadas. Las insignias globales y las con alcance por página se sobrescriben de forma independiente — sobrescribir las insignias globales no afecta a las de alcance por página, y viceversa. Si es false u omitido, las insignias proporcionadas se añadirán a las existentes.
- `update` - Si es true, las propiedades de visualización de las insignias se actualizarán desde la configuración del tenant cada vez que el usuario inicie sesión.

---
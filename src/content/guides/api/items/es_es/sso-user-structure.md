FastComments proporciona una solución SSO fácil de usar. Actualizar la información de un usuario con la integración basada en HMAC es
tan simple como hacer que el usuario cargue la página con un payload actualizado.

Sin embargo, puede ser deseable gestionar un usuario fuera de ese flujo, para mejorar la consistencia de su aplicación.

La API de Usuario SSO proporciona una forma de hacer CRUD en objetos que llamamos SSOUsers. Estos objetos son diferentes de los Usuarios regulares y
se mantienen separados por seguridad de tipos.

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
    isAccountOwner?: boolean // Admin permission - SSO users with this flag are billed as SSO Admins (separate from regular SSO users)
    isAdminAdmin?: boolean // Admin permission - SSO users with this flag are billed as SSO Admins (separate from regular SSO users)
    isCommentModeratorAdmin?: boolean // Moderator permission - SSO users with this flag are billed as SSO Moderators (separate from regular SSO users)
    /** If null, Access Control will not be applied to the user. If an empty list, this user will not be able to see any pages or @mention other users. **/
    groupIds?: string[] | null
    createdFromSimpleSSO?: boolean
    /** Don't let other users see this user's activity, including comments, on their profile. Default is true to provide secure profiles by default. **/
    isProfileActivityPrivate?: boolean
    /** Don't let other users leave comments on the user's profile, or see existing profile comments. Default false. **/
    isProfileCommentsPrivate?: boolean
    /** Don't let other users send direct messages to this user. Default false. **/
    isProfileDMDisabled?: boolean
    karma?: number
    /** Optional configuration for user badges. **/
    badgeConfig?: {
        /** Array of badge IDs to assign to the user. Limited to 30 badges. Order is respected. **/
        badgeIds: string[]
        /** If true, replaces all existing displayed badges with the provided ones. If false, adds to existing badges. **/
        override?: boolean
        /** If true, updates badge display properties from tenant configuration. **/
        update?: boolean
    }
}
[inline-code-end]

### Facturación para Usuarios SSO

Los usuarios SSO se facturan de manera diferente según sus banderas de permisos:

- **Usuarios SSO Regulares**: Los usuarios sin permisos de administrador o moderador se facturan como usuarios SSO regulares
- **Administradores SSO**: Los usuarios con banderas `isAccountOwner` o `isAdminAdmin` se facturan por separado como Administradores SSO (misma tarifa que los administradores de inquilino regulares)
- **Moderadores SSO**: Los usuarios con bandera `isCommentModeratorAdmin` se facturan por separado como Moderadores SSO (misma tarifa que los moderadores regulares)

**Importante**: Para prevenir doble facturación, el sistema automáticamente deduplica usuarios SSO contra usuarios de inquilino regulares y moderadores por dirección de email. Si un usuario SSO tiene el mismo email que un usuario de inquilino regular o moderador, no serán facturados dos veces.

### Control de Acceso

Los usuarios pueden dividirse en grupos. Esto es para lo que es el campo `groupIds`, y es opcional.

### @Menciones

Por defecto `@mentions` usará `username` para buscar otros usuarios sso cuando se escribe el carácter `@`. Si se usa `displayName`, entonces los resultados que coincidan con
`username` serán ignorados cuando haya una coincidencia para `displayName`, y los resultados de búsqueda de `@mention` usarán `displayName`.

### Suscripciones

Con FastComments, los usuarios pueden suscribirse a una página haciendo clic en el icono de campana en el widget de comentarios y haciendo clic en Suscribirse.

Con un usuario regular, les enviamos emails de notificación basados en sus configuraciones de notificación.

Con Usuarios SSO, dividimos esto para compatibilidad hacia atrás. Los usuarios solo recibirán estos emails de notificación de suscripción adicionales
si establece `optedInSubscriptionNotifications` a `true`.

### Insignias

Puede asignar insignias a usuarios SSO usando la propiedad `badgeConfig`. Las insignias son indicadores visuales que aparecen junto al nombre de un usuario en los comentarios.

- `badgeIds` - Un array de IDs de insignias para asignar al usuario. Estos deben ser IDs de insignias válidos creados en su cuenta de FastComments. Limitado a 30 insignias.
- `override` - Si es verdadero, todas las insignias existentes mostradas en comentarios serán reemplazadas con las proporcionadas. Si es falso u omitido, las insignias proporcionadas se agregarán a cualquier insignia existente.
- `update` - Si es verdadero, las propiedades de visualización de insignias se actualizarán desde la configuración del inquilino cada vez que el usuario inicie sesión.

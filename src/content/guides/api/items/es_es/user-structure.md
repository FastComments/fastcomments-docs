`User` es un objeto que representa el denominador más común de todos los usuarios.

Tenga en cuenta que en FastComments tenemos varios casos de uso diferentes para usuarios:

- SSO Seguro
- SSO Simple
- Usuarios de Inquilino (Por ejemplo: Administradores)
- Comentaristas

Esta API es para **Comentaristas** y usuarios creados a través de **SSO Simple**. Básicamente, cualquier usuario creado
a través de su sitio puede ser accedido a través de esta API. Los Usuarios de Inquilino también pueden ser obtenidos de esta manera, pero obtendrá más información interactuando con la API `/tenant-users/`.

Para `SSO Seguro` por favor use la API `/sso-users/`.

No puede actualizar estos tipos de usuarios. Ellos crearon su cuenta a través de su sitio, por lo que proporcionamos acceso básico de solo lectura, pero
no puede realizar cambios. Si desea tener este tipo de flujo - necesita configurar `SSO Seguro`.

La estructura del objeto `User` es la siguiente:

[inline-code-attrs-start title = 'Estructura de User'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface User {
    /** This is also the id used as userId on comment objects. **/
    id: string
    username: string
    /** A link to the commenter's blog, for example. **/
    websiteUrl?: string
    email: string
    signUpDate: number
    createdFromUrlId: string
    createdFromTenantId: string
    avatarSrc?: string
    locale: FastCommentsLocale
    displayLabel?: string
    karma?: number
}
[inline-code-end]

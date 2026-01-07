Un objeto `DomainConfig` representa la configuración para un dominio de un inquilino.

La estructura del objeto `DomainConfig` es la siguiente:

[inline-code-attrs-start title = 'Estructura de Configuración de Dominio'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfig {
    /** A domain, not a URL, like "fastcomments.com" or "www.example.com". Subdomain may be included if limiting to a subdomain is desired. Max 1000 characters. **/
    domain: string
    /** The From-Name used when sending emails. **/
    emailFromName?: string
    /** The From-Email used when sending emails. Ensure SPF is setup to allow mail.fastcomments.com to send emails as the domain used in this attribute. **/
    emailFromEmail?: string
    /** READONLY. When the object was created. **/
    createdAt: string
    /** The logo related to this domain. Used in emails. Use HTTPS. **/
    logoSrc?: string
    /** A smaller logo related to this domain. Use HTTPS. **/
    logoSrc100px?: string
    /** SSO ONLY. The URL used in the footer of every email sent. Supports a "[userId]" variable. **/
    footerUnsubscribeURL?: string
    /** SSO ONLY. The headers used in of every email sent. Useful for example for setting unsubscribe related headers to improve delivery. The List-Unsubscribe entry in this Record, if it exists, supports a "[userId]" variable. **/
    emailHeaders?: Record<string, string>
    /** Disable all unsubscribe links. Not recommended, may hurt delivery rates. **/
    disableUnsubscribeLinks?: boolean
    /** DKIM Configuration. **/
    dkim?: DomainConfigDKIM
}
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de Configuración DKIM'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigDKIM {
    /** The domain name in your DKIM record. **/
    domainName: string
    /** The DKIM key selector to use. **/
    keySelector: string
    /** Your private key. Start with -----BEGIN PRIVATE KEY----- and end with -----END PRIVATE KEY----- **/
    privateKey: string
}
[inline-code-end]

### Para Autenticación

La Configuración de Dominio se usa para determinar qué sitios pueden alojar el widget de FastComments para su cuenta. Esta es una forma básica
de autenticación, lo que significa que agregar o eliminar cualquier Configuración de Dominio puede impactar la disponibilidad de su instalación de FastComments
en producción.

No elimine ni actualice la propiedad `domain` de una `Domain Config` para un dominio que está actualmente en uso a menos que deshabilitar ese dominio sea intencional.

Esto tiene el mismo comportamiento que eliminar un dominio desde [/auth/my-account/configure-domains](https://fastcomments.com/auth/my-account/configure-domains).

También tenga en cuenta que eliminar un dominio desde la UI de `Mis Dominios` eliminará cualquier configuración correspondiente para ese dominio que pueda haber sido agregada a través de esta UI.

### Para Personalización de Email

El enlace de cancelación de suscripción en el pie de página del email, y la función de cancelación de suscripción con un clic ofrecida por muchos clientes de correo electrónico, se pueden configurar a través de esta API definiendo `footerUnsubscribeURL` y `emailHeaders`, respectivamente.

### Para DKIM

Después de definir sus registros DNS de DKIM, simplemente actualice el DomainConfig con su configuración DKIM usando la estructura definida.

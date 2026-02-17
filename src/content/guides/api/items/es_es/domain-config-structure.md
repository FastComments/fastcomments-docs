Un objeto `DomainConfig` representa la configuración de un dominio para un tenant.

La estructura del objeto `DomainConfig` es la siguiente:

[inline-code-attrs-start title = 'Estructura de DomainConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfig {
    /** Un dominio, no una URL, como "fastcomments.com" o "www.example.com". Se puede incluir el subdominio si se desea limitar a un subdominio. Máximo 1000 caracteres. **/
    domain: string
    /** El nombre del remitente (From-Name) usado al enviar correos. **/
    emailFromName?: string
    /** El From-Email usado al enviar correos. Asegúrate de que SPF esté configurado para permitir que mail.fastcomments.com envíe correos como el dominio usado en este atributo. **/
    emailFromEmail?: string
    /** SOLO LECTURA. Cuándo se creó el objeto. **/
    createdAt: string
    /** El logotipo relacionado con este dominio. Usado en correos electrónicos. Usa HTTPS. **/
    logoSrc?: string
    /** Un logotipo más pequeño relacionado con este dominio. Usa HTTPS. **/
    logoSrc100px?: string
    /** SOLO SSO. La URL usada en el pie de cada correo enviado. Admite la variable "[userId]". **/
    footerUnsubscribeURL?: string
    /** SOLO SSO. Las cabeceras usadas en cada correo enviado. Útil, por ejemplo, para establecer cabeceras relacionadas con la baja para mejorar la entrega. La entrada List-Unsubscribe en este Record, si existe, admite la variable "[userId]". **/
    emailHeaders?: Record<string, string>
    /** Desactivar todos los enlaces de baja. No recomendado, puede perjudicar las tasas de entrega. **/
    disableUnsubscribeLinks?: boolean
    /** Configuración DKIM. **/
    dkim?: DomainConfigDKIM
}
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de configuración DKIM'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigDKIM {
    /** El nombre de dominio en tu registro DKIM. **/
    domainName: string
    /** El selector de clave DKIM a usar. **/
    keySelector: string
    /** La clave pública, en formato PEM. Devuelta en respuestas GET. **/
    publicKey: string
    /** @deprecated Ya no se devuelve en las respuestas de la API. Aceptada al escribir para compatibilidad hacia atrás. **/
    privateKey?: string
}
[inline-code-end]

### Para autenticación

La Configuración de Dominio se utiliza para determinar qué sitios pueden alojar el widget de FastComments para tu cuenta. Esta es una forma básica
de autenticación, lo que significa que agregar o eliminar cualquier Configuración de Dominio puede afectar la disponibilidad de tu instalación de FastComments
en producción.

No elimines ni actualices la propiedad `domain` de un `Domain Config` para un dominio que está en uso actualmente a menos que se pretenda desactivar ese dominio.

Esto tiene el mismo comportamiento que eliminar un dominio desde [/auth/my-account/configure-domains](https://fastcomments.com/auth/my-account/configure-domains).

Ten en cuenta también que eliminar un dominio desde la interfaz `My Domains` quitará cualquier configuración correspondiente para ese dominio que haya sido añadida mediante esta interfaz.

### Para la personalización de correos electrónicos

El enlace de baja en el pie del correo electrónico, y la función de baja con un solo clic ofrecida por muchos clientes de correo, se pueden configurar a través de esta API definiendo `footerUnsubscribeURL` y `emailHeaders`, respectivamente.

### Para DKIM

Después de definir tus registros DNS DKIM, simplemente actualiza el DomainConfig con tu configuración DKIM usando la estructura definida. 

---
SAML (Security Assertion Markup Language) es un estándar abierto basado en XML para el intercambio de datos de autenticación y autorización entre partes, particularmente entre un proveedor de identidad (IdP) y un proveedor de servicios (SP).

### Cómo funciona SAML

SAML permite el inicio de sesión único (SSO) al permitir que los usuarios se autentiquen una vez con su proveedor de identidad y luego accedan a múltiples aplicaciones sin volver a ingresar credenciales. Cuando un usuario intenta acceder a FastComments:

1. **Solicitud de autenticación**: FastComments redirige al usuario a su proveedor de identidad
2. **Autenticación del usuario**: El usuario se autentica con su IdP (p. ej., Active Directory, Okta, Azure AD)
3. **Respuesta SAML**: El IdP envía una aserción SAML firmada de vuelta a FastComments
4. **Acceso del usuario**: FastComments valida la aserción y concede acceso al usuario autenticado

### Beneficios de SAML

- **Seguridad mejorada**: La autenticación centralizada reduce los riesgos de seguridad relacionados con contraseñas
- **Mejor experiencia de usuario**: Los usuarios inician sesión una vez y acceden a múltiples aplicaciones sin interrupciones
- **Cumplimiento**: Ayuda a cumplir los requisitos regulatorios para el control de acceso y las auditorías
- **Control administrativo**: Los administradores de TI mantienen la gestión centralizada de usuarios

### Soporte de SAML 2.0

FastComments implementa SAML 2.0, la versión más ampliamente adoptada del estándar SAML. Nuestra implementación admite:

- HTTP-POST and HTTP-Redirect bindings
- Respuestas y aserciones SAML firmadas
- Aserciones cifradas (opcional)
- Múltiples algoritmos de firma y digest
- Varios formatos de identificador de nombre
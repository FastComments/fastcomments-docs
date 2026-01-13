FastComments ofrece tanto SSO como autenticación SAML. Entender las diferencias le ayuda a elegir el enfoque correcto para su organización.

### SSO simple/SSO seguro

FastComments ofrece dos flujos SSO diferentes para autenticarse en el widget de comentarios a través de su sitio.
Esto es diferente de SAML y no requiere SAML. En su lugar, SSO simple simplemente requiere pasar un objeto al
widget de comentarios, mientras que SSO seguro hace esto y además aplica hashing al payload con una API key.

SAML, por otro lado, autentica al usuario en todo el producto (basado en sus permisos) *así como* en el widget de comentarios
(si tienen habilitadas las cookies de terceros para nuestro dominio).

### Autenticación SAML

SAML es un protocolo de autenticación de nivel empresarial que ofrece capacidades de seguridad e integración más robustas:

- **Implementation**: Requiere la configuración del Identity Provider (IdP) y el intercambio de certificados
- **Security**: Utiliza assertions XML firmadas y soporta cifrado
- **Use Case**: Ideal para empresas con infraestructura SAML existente (Active Directory, Okta, etc.)
- **Setup Complexity**: Más complejo: requiere la configuración del IdP y la gestión de certificados
- **Enterprise Features**: Mapeo avanzado de roles, gestión centralizada de usuarios, registros de auditoría

### Cuándo elegir SAML

Considere la autenticación SAML si su organización:

- Ya utiliza un proveedor de identidad compatible con SAML (Okta, Azure AD, ADFS, etc.)
- Requiere seguridad y cumplimiento de nivel empresarial
- Necesita gestión centralizada de usuarios y control de acceso
- Tiene múltiples aplicaciones que usan SAML para la autenticación
- Requiere registros de auditoría detallados y reportes de seguridad

### Cuándo elegir SSO simple o seguro

Nuestras soluciones SSO centradas en el widget pueden ser suficientes si usted:

- Tiene un sistema de autenticación personalizado
- Necesita una implementación rápida con configuración mínima
- No requiere integración con un proveedor de identidad empresarial
- Quiere controlar los datos de usuario directamente desde su aplicación
- Tiene requisitos de seguridad más simples

SSO simple y SSO seguro se usan comúnmente para portales en línea, blogs, etc., donde el usuario ya tiene una cuenta *a través de su sitio o app*
pero no necesariamente utiliza SAML.
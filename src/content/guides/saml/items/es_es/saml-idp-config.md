Después de configurar SAML en FastComments, necesita configurar FastComments como Proveedor de Servicios en su proveedor de identidad.

### Configuración general del IdP

La mayoría de proveedores de identidad requieren la siguiente información para agregar FastComments como aplicación SAML:

#### Información requerida del Proveedor de Servicios

Estos valores se generan automáticamente y se muestran en la página de configuración SAML de FastComments:

**SP Entity ID / Audience**
- Format: `https://fastcomments.com/saml/{your-tenant-id}`
- Esto identifica de forma única su instancia de FastComments

**Assertion Consumer Service (ACS) URL**
- Format: `https://fastcomments.com/saml/callback/{your-tenant-id}`
- Dónde su IdP envía las respuestas SAML tras la autenticación

**SP Metadata URL** *(si su IdP lo admite)*
- Format: `https://fastcomments.com/saml/metadata/{your-tenant-id}`
- Proporciona la configuración SAML completa en formato XML

**SAML Login URL**
- Format: `https://fastcomments.com/saml/login/{your-tenant-id}`
- Enlace directo para iniciar la autenticación SAML

### Atributos SAML requeridos

Configure su proveedor de identidad para enviar estos atributos con las respuestas SAML:

#### Atributos esenciales

**Dirección de correo electrónico** *(Obligatorio)*
- **Attribute Name**: `email`, `emailAddress`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/emailaddress`
- **Purpose**: Identificación única del usuario y notificaciones
- **Format**: Dirección de correo electrónico válida

#### Atributos opcionales

**First Name**
- **Attribute Names**: `firstName`, `givenName`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/givenname`
- **Purpose**: Nombre para mostrar del usuario

**Last Name**
- **Attribute Names**: `lastName`, `surname`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/surname`
- **Purpose**: Nombre para mostrar del usuario

**Roles** *(Importante para el control de acceso)*
- **Attribute Names**: `roles`, `groups`, `memberOf`, or custom attribute names
- **Purpose**: Asignación de roles y permisos en FastComments
- **Format**: array de cadenas de roles o valores separados por comas

### Configuraciones comunes de proveedores de identidad

#### Microsoft Azure AD

1. **Agregar aplicación empresarial**
   - Busque "FastComments" o cree una aplicación SAML personalizada
   - Utilice la información del SP proporcionada por FastComments

2. **Configure los atributos**
   - Email: `user.mail` or `user.userprincipalname`
   - First Name: `user.givenname`
   - Last Name: `user.surname`
   - Roles: `user.assignedroles` or directory groups

#### Okta

1. **Crear aplicación SAML**
   - Use "Create New App" y seleccione SAML 2.0
   - Configure con la información del SP de FastComments

2. **Attribute Statements**
   - Email: `user.email`
   - FirstName: `user.firstName`
   - LastName: `user.lastName`
   - Roles: `user.groups` or custom attributes

#### Google Workspace

1. **Agregar aplicación SAML**
   - Vaya a Apps > Web and mobile apps > Add App > Add custom SAML app
   - Configure con la información del SP de FastComments

2. **Attribute Mapping**
   - Email: Primary email
   - First Name: First name
   - Last Name: Last name
   - Roles: Groups or custom attributes

#### Active Directory Federation Services (ADFS)

1. **Agregar Relying Party Trust**
   - Use la URL de metadatos de FastComments o configuración manual
   - Configure la información del SP según se indique

2. **Reglas de claims**
   - Email: claim de dirección de correo electrónico
   - Name: claim Name ID
   - Roles: Pertenencia a grupos o claims personalizados

### Flexibilidad de nombres de atributos

FastComments acepta información de roles a partir de múltiples nombres de atributo para adaptarse a diferentes configuraciones de IdP:

- `roles`
- `groups`
- `memberOf`
- `role`
- `group`
- `http://schemas.microsoft.com/ws/2008/06/identity/claims/role`
- `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/role`

Esta flexibilidad garantiza la compatibilidad con diversos proveedores de identidad sin requerir convenciones específicas de nombres de atributos.

### Prueba de su configuración

Después de configurar su proveedor de identidad:

1. Guarde la configuración del IdP
2. Pruebe con una cuenta de usuario de prueba dedicada
3. Verifique que los atributos se estén enviando correctamente
4. Compruebe que los roles estén mapeados correctamente
5. Asegúrese de que el flujo de autenticación se complete correctamente

La mayoría de proveedores de identidad ofrecen herramientas de prueba SAML para validar la configuración antes de implementarla en usuarios de producción.
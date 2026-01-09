Cuando SAML está habilitado en FastComments, el sistema genera automáticamente la información del Proveedor de Servicios (SP) que necesita configurar en su proveedor de identidad.

### Acceder a la información del Proveedor de Servicios

La información del SP se muestra en la página de configuración de SAML después de habilitar la autenticación SAML. Esta información incluye todos los detalles que su proveedor de identidad necesita para establecer la relación de confianza SAML.

### Puntos finales del Proveedor de Servicios

#### ID de entidad de SP / Audiencia
**Propósito**: Identifica de forma única su instancia de FastComments como proveedor de servicios  
**Formato**: `https://fastcomments.com/saml/{your-tenant-id}`  
**Uso**: Configure esto como el ID de entidad o Audience en su IdP

Este identificador asegura que las respuestas SAML estén destinadas a su tenant específico de FastComments y evita que respuestas SAML sean aceptadas por otras instancias.

#### URL del Assertion Consumer Service (ACS)
**Propósito**: El endpoint donde su IdP envía las respuestas SAML después de la autenticación del usuario  
**Formato**: `https://fastcomments.com/saml/callback/{your-tenant-id}`  
**Uso**: Configure esto como la URL ACS o la URL de respuesta en su IdP

Aquí es donde los usuarios son redirigidos después de una autenticación exitosa con su proveedor de identidad, junto con la aserción SAML que contiene la información del usuario.

#### URL de metadatos del SP
**Propósito**: Proporciona la configuración completa de SAML en formato XML estándar  
**Formato**: `https://fastcomments.com/saml/metadata/{your-tenant-id}`  
**Uso**: Algunos IdP pueden importar automáticamente la configuración usando esta URL

La URL de metadatos contiene toda la información necesaria del SP en formato XML, lo que facilita configurar proveedores de identidad compatibles de forma automática.

#### URL de inicio de sesión SAML
**Propósito**: Enlace directo para iniciar la autenticación SAML para su tenant  
**Formato**: `https://fastcomments.com/saml/login/{your-tenant-id}`  
**Uso**: Enlace a los usuarios directamente a la autenticación SAML o para probar el flujo

Puede usar esta URL para probar la autenticación SAML o proporcionar a los usuarios un enlace directo para iniciar sesión vía SAML.

### Compatibilidad con bindings SAML

FastComments soporta los siguientes bindings SAML:

#### Binding HTTP-POST
- **Método principal**: El binding más común para las respuestas SAML  
- **Seguridad**: La respuesta SAML se envía vía HTTP POST al ACS URL  
- **Uso**: Recomendado para despliegues en producción

#### Binding HTTP-Redirect
- **Método alternativo**: Respuesta SAML enviada vía redirección HTTP  
- **Limitaciones**: Tamaño de carga útil limitado debido a las restricciones de longitud de URL  
- **Uso**: Soportado, pero se prefiere HTTP-POST

### Política de Name ID

FastComments configura la siguiente política de Name ID en las solicitudes SAML:

- **Formato predeterminado**: `urn:oasis:names:tc:SAML:1.1:nameid-format:emailAddress`  
- **Formatos alternativos**: Persistent, Transient, Unspecified (configurable)  
- **Requisito**: La dirección de correo electrónico se utiliza como identificador principal del usuario

### Atributos de la solicitud SAML

Al iniciar la autenticación SAML, FastComments envía solicitudes con estas características:

#### Firma de la solicitud
- **Estado**: Opcional (configurable)  
- **Algoritmo**: Coincide con el algoritmo de firma configurado  
- **Certificado**: Utiliza un certificado específico del tenant si la firma de solicitudes está habilitada

#### Atributos solicitados
FastComments solicita los siguientes atributos en las AuthnRequests SAML:

- **Email**: Requerido para la identificación del usuario  
- **First Name**: Opcional para fines de visualización  
- **Last Name**: Opcional para fines de visualización  
- **Roles/Groups**: Opcional para control de acceso y permisos

### Copiar información del SP

La página de configuración de SAML ofrece campos clicables que copian automáticamente la información del SP al portapapeles:

1. Haga clic en cualquier campo de información del SP (Entity ID, ACS URL, etc.)  
2. El valor se copia automáticamente al portapapeles  
3. Pegue el valor en la configuración de su proveedor de identidad  
4. Un breve resaltado indica la copia exitosa

Esto facilita transferir con precisión la información del SP a su IdP sin errores de tipeo.

### Información del certificado del SP

#### Uso del certificado
- **Propósito**: Cifra las comunicaciones y verifica la identidad del SP  
- **Rotación**: Los certificados se gestionan automáticamente por FastComments  
- **Acceso**: Los certificados públicos están disponibles a través de la URL de metadatos

#### Detalles del certificado
- **Algoritmo**: RSA-2048 o superior  
- **Validez**: Los certificados se renuevan automáticamente antes de su vencimiento  
- **Distribución**: Disponibles a través de metadatos SAML estándar

### Solución de problemas de la configuración del SP

Si su proveedor de identidad informa problemas con la información del SP:

1. **Verificar URLs**: Asegúrese de que todas las URLs usan HTTPS e incluyen el tenant ID correcto  
2. **Comprobar metadatos**: Use la URL de metadatos para verificar la configuración  
3. **Probar conectividad**: Asegúrese de que su IdP pueda alcanzar los endpoints de FastComments  
4. **Validar formato**: Confirme que su IdP soporte el formato de la información del SP

Problemas comunes incluyen:
- ID de tenant incorrecto en las URLs  
- Problemas de conectividad de red entre el IdP y FastComments  
- IdP esperando diferentes formatos de URL u opciones de configuración adicionales
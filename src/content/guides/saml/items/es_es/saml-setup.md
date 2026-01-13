Configurar la autenticación SAML en FastComments requiere tanto la configuración en tu panel de administración como la configuración en tu proveedor de identidad.

### Requisitos previos

Antes de configurar SAML, asegúrate de tener:

- Un plan FastComments Flex o Pro (SAML no está disponible en el plan Creators)
- Acceso administrativo a tu cuenta de FastComments
- Acceso administrativo a tu proveedor de identidad
- Los metadatos SAML o la información del certificado de tu IdP

### Acceder a la configuración SAML

1. Inicia sesión en tu [panel de administración de FastComments](https://fastcomments.com/auth/my-account)
2. Navega a **API/SSO Settings** en la barra lateral izquierda
3. Haz clic en el botón **SAML Config**

Si no ves el botón SAML Config, verifica que:
- Tu cuenta tenga el paquete requerido (Flex o Pro)
- Tienes permisos administrativos
- Tu usuario tiene los roles API Admin o Admin Admin

### Configuración básica de SAML

#### Habilitar la autenticación SAML

1. Marca la casilla **Enable SAML Authentication**
2. Esto activa SAML para tu tenant y hace que los campos de configuración estén disponibles

#### Campos obligatorios

**IdP Single Sign-On URL** *(Required)*
- La URL a la que se redirigirá a los usuarios para la autenticación
- Normalmente la proporciona tu proveedor de identidad
- Ejemplo: `https://your-company.okta.com/app/fastcomments/sso/saml`

**IdP X.509 Certificate** *(Required)*
- El certificado público de tu proveedor de identidad
- Se utiliza para verificar la autenticidad de las respuestas SAML
- Debe incluir el certificado completo con los marcadores BEGIN/END
- Formato de ejemplo:
```
-----BEGIN CERTIFICATE-----
MIICXjCCAcegAwIBAgIBADANBgkqhkiG9w0BAQsFADA...
-----END CERTIFICATE-----
```

#### Campos opcionales

**IdP Entity ID / Issuer**
- Identifica a tu proveedor de identidad
- Si se deja en blanco, por defecto será tu URL de FastComments
- Debe coincidir con el issuer configurado en tu IdP

### Configuración avanzada

#### Ajustes de seguridad

**Signature Algorithm**
- Por defecto SHA-256 (recomendado)
- Opciones: SHA-1, SHA-256, SHA-512
- Debe coincidir con la configuración de tu IdP

**Digest Algorithm**
- Por defecto SHA-256 (recomendado)
- Se utiliza para el cálculo del digest en las respuestas SAML
- Debe coincidir con la configuración de tu IdP

**Name ID Format**
- Por defecto el formato Email Address
- Determina cómo se formatean los identificadores de usuario
- Opciones comunes: Email Address, Persistent, Transient

#### Cifrado (opcional)

**Private Key for Decryption**
- Solo es necesario si tu IdP cifra las assertions SAML
- Pega tu clave privada utilizada para el descifrado
- La mayoría de las implementaciones no requieren cifrado de assertions

### Guardar la configuración

1. Revisa todos los ajustes para verificar su exactitud
2. Haz clic en **Save SAML Configuration**
3. El sistema validará tu configuración
4. Si tiene éxito, verás un mensaje de confirmación

### Siguientes pasos

Después de guardar tu configuración SAML en FastComments:

1. Configura tu proveedor de identidad usando la información del Proveedor de Servicios
2. Prueba el flujo de autenticación
3. Configura los roles y permisos de usuario según sea necesario

La información del Proveedor de Servicios necesaria para la configuración de tu IdP se mostrará una vez que SAML esté habilitado.
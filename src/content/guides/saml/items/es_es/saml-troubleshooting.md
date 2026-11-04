Esta guía cubre problemas comunes de autenticación SAML y sus soluciones.

### Problemas de Certificados y Seguridad

#### Error de Certificado Inválido

**Síntomas**:
- Error "Certificate validation failed" (falló la validación del certificado)
- Los usuarios no pueden completar la autenticación SAML
- Las respuestas SAML son rechazadas

**Causas Comunes**:
- El formato del certificado es incorrecto
- El certificado ha expirado
- Se proporcionó un certificado incorrecto
- Caracteres adicionales o espacios en blanco en el certificado

**Soluciones**:
1. **Verificar el Formato del Certificado**:
   - Asegúrese de que el certificado incluya los marcadores `-----BEGIN CERTIFICATE-----` y `-----END CERTIFICATE-----`
   - Elimine cualquier espacio en blanco o salto de línea adicional
   - Copie el certificado directamente desde los metadatos o la configuración del IdP

2. **Comprobar la Validez del Certificado**:
   - Verifique que el certificado no haya expirado
   - Confirme que el certificado sea para el IdP correcto
   - Use validadores de certificados en línea para comprobar el formato

3. **Volver a Descargar el Certificado**:
   - Descargue un certificado nuevo desde el IdP
   - Use la URL de metadatos del IdP si está disponible
   - Confirme que el certificado coincida con la configuración actual del IdP

#### Fallo en la Verificación de Firma

**Síntomas**:
- Errores de validación de la firma de la aserción SAML
- La autenticación falla después del inicio de sesión en el IdP
- Mensajes de error "Invalid signature" (firma inválida)

**Soluciones**:
1. **Discrepancia de Algoritmo**:
   - Compruebe que el algoritmo de firma en FastComments coincida con el del IdP
   - Pruebe diferentes algoritmos de firma (SHA-256, SHA-1, SHA-512)
   - Verifique que el algoritmo de digest coincida con la configuración del IdP

2. **Problemas de Certificado**:
   - Asegúrese de que el certificado de firma correcto esté configurado
   - Verifique que el certificado corresponda a la clave privada utilizada por el IdP
   - Compruebe si hay rotación de certificados en el IdP

### Problemas de Configuración

#### Entity ID o ACS URL Incorrectos

**Síntomas**:
- El IdP informa "Unknown Service Provider" (proveedor de servicios desconocido)
- Las respuestas SAML van a un endpoint incorrecto
- La autenticación no se completa

**Soluciones**:
1. **Verificar la Información del SP**:
   - Copie el Entity ID exacto de la configuración de FastComments
   - Asegúrese de que la ACS URL coincida con el formato: `https://fastcomments.com/saml/callback/{tenant-id}`
   - Compruebe si hay errores tipográficos en el ID del tenant

2. **Configuración del IdP**:
   - Actualice el IdP con el SP Entity ID correcto
   - Configure la ACS/Reply URL adecuada
   - Verifique la configuración de binding del IdP (se prefiere HTTP-POST)

#### Atributos Faltantes o Incorrectos

**Síntomas**:
- Usuarios creados sin los roles adecuados
- Falta información del perfil de usuario
- Errores "Email required" (correo electrónico requerido)

**Soluciones**:
1. **Atributo de Correo Electrónico**:
   - Asegúrese de que el IdP envíe el atributo de correo electrónico
   - Compruebe el mapeo de nombres de atributos (email, emailAddress, etc.)
   - Verifique que el valor del correo electrónico sea una dirección de correo válida

2. **Atributos de Roles**:
   - Confirme que el IdP envíe información de roles/grupos
   - Compruebe que los nombres de los atributos de rol coincidan con lo que espera FastComments
   - Verifique que los valores de los roles coincidan exactamente con los nombres de roles de FastComments

3. **Formato de los Atributos**:
   - Pruebe tanto el formato de array como el separado por comas para los roles
   - Asegúrese de que los valores de los atributos no tengan espacios en blanco adicionales
   - Compruebe la sensibilidad a mayúsculas y minúsculas en los nombres de los roles

### Problemas en el Flujo de Autenticación

#### Bucle de Redirección

**Síntomas**:
- El navegador se redirige sin fin entre FastComments y el IdP
- La autenticación nunca se completa
- Se muestran múltiples redirecciones en las herramientas de desarrollo del navegador

**Soluciones**:
1. **Comprobar la Configuración del SP**:
   - Verifique que el Entity ID coincida exactamente con la configuración del IdP
   - Asegúrese de que la ACS URL esté correctamente configurada en el IdP
   - Compruebe si hay barras diagonales finales en las URLs

2. **Problemas de Sesión**:
   - Borre las cookies del navegador e inténtelo de nuevo
   - Pruebe en una ventana de navegación de incógnito/privada
   - Compruebe la configuración de tiempo de espera de la sesión

#### Acceso Denegado Después de la Autenticación

**Síntomas**:
- La autenticación SAML tiene éxito
- El usuario es redirigido a FastComments
- Se muestra un error de "Access denied" (acceso denegado) o de permisos

**Soluciones**:
1. **Asignación de Roles**:
   - Verifique que el usuario tenga los roles adecuados en el IdP
   - Compruebe que el atributo de rol se esté enviando en la respuesta SAML
   - Confirme que los nombres de los roles coincidan exactamente con los requisitos de FastComments

2. **Limitaciones del Plan**:
   - Verifique que la cuenta tenga el plan Flex o Pro
   - Compruebe que la función SAML esté habilitada para el plan
   - Contacte con soporte si el plan incluye SAML pero la función no está disponible

### Problemas Específicos de Proveedores de Identidad

#### Microsoft Azure AD

**Problemas Comunes**:
- Las asignaciones de roles de la aplicación no se reflejan en los tokens
- Los claims no se envían correctamente
- Requisitos de asignación de usuarios

**Soluciones**:
- Compruebe la asignación del usuario a la aplicación de FastComments
- Verifique que los roles de la aplicación estén correctamente configurados
- Asegúrese de que el mapeo de claims incluya los atributos requeridos

#### Okta

**Problemas Comunes**:
- Los filtros de grupos no funcionan correctamente
- Declaraciones de atributos mal configuradas
- Problemas de asignación de la aplicación

**Soluciones**:
- Revise la configuración de las declaraciones de atributos
- Compruebe las reglas de asignación y filtrado de grupos
- Verifique que la aplicación esté asignada a los usuarios/grupos adecuados

#### Google Workspace

**Problemas Comunes**:
- Los atributos personalizados no se mapean correctamente
- No se envía la pertenencia a grupos
- Errores de configuración de la aplicación SAML

**Soluciones**:
- Configure un esquema personalizado para los atributos de rol
- Compruebe la propagación de la pertenencia a grupos
- Verifique el mapeo de atributos de la aplicación SAML

### Problemas de Red y Conectividad

#### Errores de Tiempo de Espera

**Síntomas**:
- El proceso de autenticación agota el tiempo de espera
- Errores "Request timeout" (tiempo de espera de la solicitud) o similares
- Flujo de autenticación lento

**Soluciones**:
1. **Conectividad de Red**:
   - Compruebe que las reglas del firewall permitan la comunicación con FastComments
   - Verifique la resolución DNS de fastcomments.com
   - Pruebe la conectividad de red desde el IdP hasta FastComments

2. **Problemas de Rendimiento**:
   - Compruebe los tiempos de respuesta del IdP
   - Verifique que la validación de la cadena de certificados no sea lenta
   - Considere la latencia de red entre el IdP y los usuarios

#### Problemas de SSL/TLS

**Síntomas**:
- Advertencias de certificado durante la autenticación
- Fallos en el handshake SSL
- Errores "Secure connection failed" (falló la conexión segura)

**Soluciones**:
- Asegúrese de que todos los endpoints SAML usen HTTPS
- Compruebe la validez de los certificados de todos los dominios involucrados
- Verifique la compatibilidad de la versión de TLS

### Depuración y Registro

#### Habilitar Información de Depuración

1. **Herramientas de Desarrollo del Navegador**:
   - Supervise la pestaña Network durante el flujo SAML
   - Compruebe la Console en busca de errores de JavaScript
   - Examine las solicitudes POST de SAML (si son visibles)

2. **Registros del IdP**:
   - Habilite la depuración SAML en su IdP
   - Revise los registros del IdP para ver los detalles de las solicitudes/respuestas SAML
   - Compruebe si hay problemas de mapeo de atributos

#### Mensajes de Registro Comunes

**Registros de FastComments**:
- "SAML config not found" - SAML no está habilitado o está mal configurado
- "Invalid certificate" - Falló la validación del certificado
- "Missing email attribute" - No se proporcionó el correo electrónico requerido en la respuesta SAML

**Registros del IdP**:
- "Unknown service provider" - Discrepancia en el Entity ID
- "Invalid ACS URL" - La URL del Assertion Consumer Service es incorrecta
- "User not assigned" - El usuario no tiene acceso a la aplicación SAML

### Cómo Obtener Ayuda

#### Información a Recopilar

Al contactar con soporte, proporcione:
- Mensajes de error exactos y marcas de tiempo
- Detalles de la configuración SAML (sin datos sensibles)
- Tipo y versión del IdP
- Pasos para reproducir el problema
- Información del navegador y de la red

#### Soporte de FastComments

Para problemas relacionados con SAML:
1. Use el [portal de soporte](https://fastcomments.com/auth/my-account/help)
2. Incluya el ID del tenant y los correos electrónicos de los usuarios afectados
3. Proporcione los mensajes de error y los detalles de configuración
4. Especifique el tipo de IdP y el enfoque de configuración

#### Soporte del IdP

Para problemas específicos del IdP:
- Consulte la documentación del IdP para la resolución de problemas de SAML
- Use los canales de soporte del IdP para problemas de configuración
- Aproveche los foros de la comunidad del IdP para problemas comunes

### Consejos de Prevención

#### Mejores Prácticas

1. **Probar a Fondo**:
   - Pruebe los cambios de configuración en un entorno que no sea de producción
   - Verifique con varios usuarios de prueba
   - Documente las configuraciones que funcionan

2. **Supervisar Regularmente**:
   - Configure la supervisión de fallos de autenticación SAML
   - Revise las fechas de expiración de los certificados
   - Supervise los cambios de configuración del IdP

3. **Documentación**:
   - Mantenga documentación de la configuración SAML
   - Documente cualquier configuración personalizada o solución alternativa
   - Guarde la información de contacto de los administradores del IdP

#### Mantenimiento Proactivo

1. **Gestión de Certificados**:
   - Supervise las fechas de expiración de los certificados
   - Planifique los procedimientos de rotación de certificados
   - Pruebe las actualizaciones de certificados antes de la expiración

2. **Revisiones de Configuración**:
   - Revise regularmente la configuración SAML
   - Verifique que la configuración del IdP siga actualizada
   - Actualice la documentación a medida que se realicen cambios
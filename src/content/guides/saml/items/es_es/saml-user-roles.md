FastComments asigna los roles de usuario SAML a permisos internos, permitiendo el control de acceso basado en roles para su organización.

### Sistema de Roles de FastComments

FastComments utiliza un sistema de permisos basado en roles donde los usuarios pueden tener uno o más roles que determinan sus niveles de acceso y capacidades.

### Roles disponibles de FastComments

#### Roles administrativos

**fc-account-owner**
- **Permisos**: Acceso administrativo completo
- **Capacidades**: Todas las funciones, gestión de facturación, gestión de usuarios
- **Caso de uso**: Administradores principales de la cuenta y propietarios

**fc-admin-admin**  
- **Permisos**: Acceso administrativo a la mayoría de funciones
- **Capacidades**: Gestión de usuarios, configuración, moderación. **Puede administrar a otros administradores.**
- **Caso de uso**: Administradores secundarios y personal de TI

**fc-billing-admin**
- **Permisos**: Gestión de facturación y suscripciones
- **Capacidades**: Métodos de pago, facturas, cambios de suscripción
- **Caso de uso**: Miembros del equipo financiero y contactos de facturación

#### Roles especializados

**fc-analytics-admin**
- **Permisos**: Acceso a analíticas e informes
- **Capacidades**: Ver estadísticas del sitio, datos de participación de usuarios
- **Caso de uso**: Equipos de marketing y analistas de datos

**fc-api-admin**
- **Permisos**: Acceso y gestión de la API
- **Capacidades**: Credenciales de API, configuración de webhooks
- **Caso de uso**: Desarrolladores e integradores técnicos

**fc-moderator**
- **Permisos**: Capacidades de moderación de comentarios
- **Capacidades**: Aprobar/rechazar comentarios, gestionar spam
- **Caso de uso**: Moderadores de la comunidad y gestores de contenido

### Configuración de mapeo de roles

#### Fuentes de atributos SAML

FastComments acepta información de roles desde varios nombres de atributos SAML para asegurar compatibilidad con diferentes proveedores de identidades:

**Nombres de atributos estándar**:
- `roles`
- `groups` 
- `memberOf`
- `role`
- `group`

**Atributos de Microsoft/ADFS**:
- `http://schemas.microsoft.com/ws/2008/06/identity/claims/role`
- `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/role`

#### Formatos de roles soportados

**Formato Array** *(Preferido)*:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin</saml:AttributeValue>
    <saml:AttributeValue>fc-moderator</saml:AttributeValue>
</saml:Attribute>
```

**Formato separado por comas**:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin,fc-moderator</saml:AttributeValue>
</saml:Attribute>
```

**Formato de rol único**:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin</saml:AttributeValue>
</saml:Attribute>
```

### Configuración de roles en el proveedor de identidades

#### Microsoft Azure AD

1. **Configuración de roles de la aplicación**:
   - Defina los roles de FastComments en su aplicación de Azure AD
   - Asigne usuarios a los roles de la aplicación correspondientes
   - Configure las claims para incluir los roles asignados

2. **Mapeo de atributos**:
   ```
   Attribute Name: roles
   Source Attribute: user.assignedroles
   ```

#### Okta

1. **Asignación de grupos**:
   - Cree grupos que coincidan con los nombres de roles de FastComments
   - Asigne usuarios a los grupos correspondientes
   - Configure las declaraciones de atributos

2. **Declaración de atributos**:
   ```
   Name: roles
   Value: user.groups
   Filter: Starts with "fc-"
   ```

#### Google Workspace

1. **Mapeo de grupos**:
   - Cree unidades organizativas o grupos
   - Nombren los grupos con los prefijos de roles de FastComments
   - Configure el mapeo de atributos

2. **Atributos personalizados**:
   ```
   Attribute Name: roles
   Value: Groups or custom schema attribute
   ```

### Comportamiento predeterminado de usuario

#### Usuarios sin roles

Cuando un usuario SAML no tiene roles o tiene roles no reconocidos:
- El usuario se crea como un comentarista estándar
- No se concede acceso administrativo
- Puede publicar y administrar sus propios comentarios
- No puede acceder a las funciones del panel de administración

#### Herencia de roles

- Los usuarios pueden tener múltiples roles simultáneamente
- Los permisos son acumulativos (se aplica el nivel de permiso más alto)
- Los cambios de rol en el IdP se reflejan en el siguiente inicio de sesión

### Gestión de usuarios SAML

#### Creación de usuario

Cuando un usuario inicia sesión vía SAML por primera vez:
1. **Cuenta de usuario**: Se crea automáticamente con el correo electrónico como identificador
2. **Asignación de roles**: Se aplican roles basados en los atributos SAML
3. **Información de perfil**: Nombre y apellidos se completan si se proporcionan
4. **Activación de permisos**: Los roles se activan de inmediato

#### Actualizaciones de roles

Los usuarios SAML existentes reciben actualizaciones de roles:
1. **Desencadenante de inicio de sesión**: Las actualizaciones de roles ocurren durante cada inicio de sesión SAML
2. **Efecto inmediato**: Los nuevos permisos se aplican de inmediato
3. **Eliminación de roles**: Los roles eliminados se revocan automáticamente
4. **Rastro de auditoría**: Los cambios de roles se registran en los registros de auditoría

### Mapeo de roles personalizado

#### Personalización empresarial

Para clientes empresariales con requisitos específicos:
- Los nombres de roles personalizados pueden mapearse a los permisos de FastComments
- Se pueden implementar jerarquías de roles complejas
- Se pueden configurar controles de acceso específicos por departamento

Contacte con el soporte de FastComments para configuraciones de mapeo de roles personalizadas.

#### Validación de roles

FastComments valida los roles entrantes:
- Los roles no reconocidos se ignoran (no se rechazan)
- Los atributos de rol malformados se registran para resolución de problemas
- Los usuarios mantienen los roles existentes si la aserción SAML carece de información de roles

### Buenas prácticas

#### Gestión de roles

1. **Principio del menor privilegio**: Asigne los permisos mínimos necesarios
2. **Auditorías periódicas**: Revise los roles y accesos de los usuarios periódicamente  
3. **Nomenclatura clara**: Use nombres de grupos descriptivos en su IdP
4. **Documentación**: Mantenga documentación de las asignaciones de roles

#### Consideraciones de seguridad

1. **Atributos de roles**: Asegúrese de que los atributos de roles estén debidamente protegidos en las respuestas SAML
2. **Validación de atributos**: Verifique que solo los sistemas autorizados puedan asignar roles
3. **Revisiones de acceso**: Revise regularmente las asignaciones de roles administrativos
4. **Supervisión**: Monitorice los cambios de roles y las acciones administrativas

### Solución de problemas de roles

#### Problemas comunes

**Roles no aplicados**:
- Compruebe que los nombres de atributos SAML coincidan con los formatos soportados
- Verifique que el IdP esté enviando la información de roles
- Confirme que los valores de rol coincidan exactamente con los nombres de roles de FastComments

**Acceso denegado**:
- Verifique que el usuario tenga el rol apropiado asignado en el IdP
- Compruebe la ortografía del rol y la sensibilidad a mayúsculas y minúsculas
- Confirme que el rol esté correctamente formateado en la respuesta SAML

**Permisos faltantes**:
- Revise las definiciones de roles y los permisos requeridos
- Compruebe si hay asignaciones de roles en conflicto
- Verifique que el usuario haya iniciado sesión después de los cambios de rol
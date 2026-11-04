La integración de LTI 1.3 de FastComments sigue el principio de mínimo privilegio: utiliza solo las claims de lanzamiento necesarias para identificar al usuario, adjuntar comentarios al curso y recurso correctos, y aplicar permisos basados en roles.

El resto de esta página detalla cada claim que consume la integración, cada servicio de LTI Advantage que no solicita y cada categoría de datos que no recopila. Los revisores de seguridad y adquisiciones pueden extraer respuestas directamente de las tablas siguientes.

## Elementos de Datos Recibidos del LMS

Cada lanzamiento de LTI 1.3 lleva un JWT firmado por el LMS. FastComments extrae las siguientes claims de ese JWT y no usa nada más:

| Campo | Claim de LTI | Propósito | Requerido | Almacenado |
|-------|-----------|---------|----------|--------|
| Identificador de usuario | `sub` | Identifica al usuario de forma consistente entre lanzamientos para que la misma persona resuelva al mismo usuario SSO de FastComments | Sí | Sí, como parte de un ID SSO interno estable |
| Nombre para mostrar | `name` | Atribución mostrada junto a los comentarios del usuario | Sí (usa "LMS User" como alternativa si está ausente) | Sí |
| Correo electrónico | `email` | Coincidencia de cuentas, notificaciones, moderación, correspondencia de soporte | Opcional (la integración funciona sin él) | Sí cuando se proporciona |
| URL del avatar | `picture` | Mostrado en los comentarios del usuario | Opcional | Solo la URL; FastComments no descarga ni rehospeda la imagen |
| Roles | `https://purl.imsglobal.org/spec/lti/claim/roles` | Determina si el usuario es administrador, instructor (moderador) o estudiante | Sí | Flags derivados `isAdmin` / `isModerator` en la sesión SSO |
| Contexto del curso | `https://purl.imsglobal.org/spec/lti/claim/context` (`id`, `title`) | Asocia el hilo de comentarios con el curso correcto del LMS | Sí | Sí, como parte del identificador de página resuelto |
| Enlace de recurso | `https://purl.imsglobal.org/spec/lti/claim/resource_link` (`id`) | Asocia los comentarios con la actividad o ubicación de herramienta correcta dentro del curso | Sí cuando está presente | Sí, como parte del identificador de página resuelto |
| ID de despliegue | `https://purl.imsglobal.org/spec/lti/claim/deployment_id` | Enruta el lanzamiento a la configuración de tenant de FastComments correcta | Sí | Sí, en el registro de configuración LTI de FastComments |

## Claims y Scopes Declarados en el Registro

Durante el Registro Dinámico de LTI 1.3, FastComments se registra con `scope: ""` (sin scopes OAuth adicionales) y declara solo estas claims de OpenID Connect:

`iss`, `sub`, `name`, `email`, `picture`

Registra dos tipos de mensaje:

- `LtiResourceLinkRequest` - el lanzamiento estándar del curso hacia FastComments.
- `LtiDeepLinkingRequest` - permite a los instructores colocar la herramienta de FastComments dentro de un curso.

No se solicitan tokens de acceso adicionales al LMS.

## Servicios de LTI Advantage No Solicitados

| Servicio / scope | ¿Solicitado? | Razón |
|------------------|------------|--------|
| Names and Role Provisioning Services (NRPS) | No | La integración no necesita una lista de estudiantes del curso; la identidad del usuario llega con cada lanzamiento |
| Assignment and Grade Services (AGS) - scopes lineitem, score, result | No | La integración no es consciente del libro de calificaciones |
| Deep Linking más allá del retorno estándar de colocación | Sin datos adicionales | Deep linking se usa solo para la colocación de la herramienta por parte del instructor; no se enumera contenido del curso |

## Datos No Recopilados

Más allá de LTI en sí, FastComments no solicita ni recibe lo siguiente del LMS o del usuario:

| Categoría | ¿Recopilado? |
|----------|------------|
| Calificaciones de estudiantes | No |
| Entregas de tareas | No |
| Registros de asistencia | No |
| Listas completas de estudiantes del curso | No |
| Identificadores gubernamentales | No |
| Fecha de nacimiento | No |
| Dirección postal o número de teléfono | No |
| Información financiera | No |
| Credenciales de administrador del LMS | No |

## Límites de Acceso

- FastComments solo recibe datos dentro de un lanzamiento de LTI 1.3 autorizado y firmado por las claves registradas del LMS. La integración no llama de vuelta al LMS para obtener información adicional.
- Los tokens de lanzamiento son de un solo uso y de corta duración. Los tokens reutilizados o expirados son rechazados.
- Los administradores del LMS controlan dónde se despliega la herramienta dentro de su plataforma. D2L Brightspace, por ejemplo, admite el scoping por unidad organizativa por despliegue y configuraciones de seguridad por despliegue, lo que permite a los administradores restringir la herramienta a cursos o unidades organizativas específicas en lugar de hacerla disponible globalmente. Moodle, Blackboard, Sakai y Schoology ofrecen controles equivalentes por despliegue en sus implementaciones de LTI 1.3.

## Almacenamiento y Retención

FastComments retiene los datos derivados de LTI durante la duración del servicio de comentarios activo y de acuerdo con las configuraciones de retención definidas por el cliente. Los datos de comentarios se almacenan en almacenamiento de producción cifrado en reposo. En caso de terminación de la cuenta o solicitud escrita de eliminación, FastComments elimina o anonimiza los datos del cliente según el acuerdo aplicable.

Para obtener detalles completos sobre el almacenamiento y el manejo de datos, consulte la <a href="https://fastcomments.com/privacy-policy" target="_blank">Política de Privacidad de FastComments</a>.

## Cadencia de Revisión

Cualquier nueva función de LTI que requiera claims, scopes o servicios de LTI Advantage adicionales se revisa antes de su lanzamiento para confirmar que el acceso solicitado es necesario y proporcional a la función que se está publicando.

## Declaración Breve para Cuestionarios de Seguridad

> FastComments aplica el mínimo privilegio y la minimización de datos a su integración de LTI 1.3. La integración utiliza solo las claims de lanzamiento de LTI necesarias para autenticar al usuario (`sub`, `name`, `email`, `picture`), determinar su rol e identificar el curso y recurso al que pertenecen los comentarios. FastComments no solicita Names and Role Provisioning Services, Assignment and Grade Services, datos del libro de calificaciones, asistencia, listas completas de estudiantes ni acceso administrativo al LMS. Los administradores del LMS mantienen el control sobre en qué unidades organizativas, cursos y despliegues está disponible la herramienta.
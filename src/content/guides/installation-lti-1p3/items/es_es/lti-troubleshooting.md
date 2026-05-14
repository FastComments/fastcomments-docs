#### "Token de registro no encontrado, caducado o ya usado"

El token en tu URL de registro es válido por 30 minutos y solo puede usarse una vez. Si tu LMS tardó más que eso, o si se reintentó el registro después de haber tenido éxito, el token será rechazado. Genera una URL nueva en la página de Configuración LTI 1.3 de FastComments y comienza de nuevo.

#### "La plataforma rechazó el registro"

Tu LMS rechazó el handshake de registro. Las causas más comunes:

- **Herramienta ya registrada con el mismo nombre de cliente.** Algunas plataformas (en particular D2L) rechazan un segundo registro de "FastComments" hasta que se elimine el anterior. Elimina la herramienta antigua en tu LMS y vuelve a intentarlo.
- **Campo incorrecto en el LMS.** Asegúrate de haber pegado la URL en el campo **registration / tool initiation registration endpoint**, no en el campo de launch URL ni en el de login URL.
- **El LMS no admite realmente Dynamic Registration.** Versiones antiguas de Moodle y Blackboard anuncian LTI 1.3 pero solo permiten la configuración manual. Consulta la documentación de tu plataforma.

#### "No se pudo obtener la configuración de la plataforma"

FastComments no pudo leer el documento openid-configuration de tu LMS. Esto es raro y normalmente significa que el LMS proporcionó una discovery URL malformada o inaccesible. Contacta con el soporte de tu LMS.

#### El lanzamiento muestra "Configuration not found"

O bien la configuración en FastComments fue eliminada, o el lanzamiento provino de un par `iss`/`client_id` que no reconocemos. Si eliminaste y volviste a registrar, indica a tu LMS que elimine y vuelva a añadir la herramienta FastComments para que obtenga el nuevo client_id.

#### El lanzamiento muestra "Despliegue no registrado"

Iniciaste FastComments desde un despliegue de Brightspace/Moodle/Blackboard distinto del que se usó en el primer lanzamiento. FastComments fija el `deployment_id` en el primer lanzamiento como verificación de seguridad. Para agregar un nuevo despliegue bajo el mismo cliente, contacta con soporte - añadiremos el deployment ID a la configuración.

#### El lanzamiento muestra "Unsupported message_type"

El LMS envió un mensaje LTI que FastComments no maneja (p. ej. `LtiSubmissionReviewRequest`). FastComments solo admite los flujos estándar de resource-link launch y deep-linking. Ponte en contacto si necesitas que se añada un tipo de mensaje específico.

#### El iframe no se redimensiona

La mayoría de los LMS ajusta automáticamente el tamaño de los iframes LTI. Si el tuyo no lo hace, comprueba que los ajustes de lanzamiento del LMS permitan que la herramienta envíe eventos postMessage al frame padre. FastComments emite mensajes de redimensionamiento tanto estilo Canvas (`lti.frameResize`) como según la especificación IMS (`org.imsglobal.lti.frameResize`).
---
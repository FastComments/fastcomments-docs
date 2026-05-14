#### "Token de registro no encontrado, caducado o ya usado"

El token en tu URL de registro (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">consíguelo aquí</a>) es válido durante 30 minutos y solo puede usarse una vez. Si tu LMS tardó más que eso, o si el registro se reintentó después de haberse completado, el token será rechazado. Genera una URL nueva en la página de Configuración LTI 1.3 de FastComments y empieza de nuevo.

#### "La plataforma rechazó el registro"

Tu LMS rechazó el apretón de manos de registro. Las causas más comunes:

- **La herramienta ya está registrada con el mismo nombre de cliente.** Algunas plataformas (en particular D2L) rechazan un segundo registro de "FastComments" hasta que se elimine el anterior. Elimina la herramienta antigua en tu LMS y vuelve a intentarlo.
- **Campo incorrecto en el LMS.** Asegúrate de pegar la URL en el campo **registration / tool initiation registration endpoint**, no en el campo launch URL ni en el campo login URL.
- **El LMS en realidad no soporta el Registro Dinámico.** Versiones antiguas de Moodle y Blackboard anuncian LTI 1.3 pero solo permiten la configuración manual. Revisa la documentación de tu plataforma.

#### "No se pudo obtener la configuración de la plataforma"

FastComments no pudo leer el documento openid-configuration de tu LMS. Esto es raro y suele significar que el LMS proporcionó una URL de descubrimiento malformada o inaccesible. Contacta con el soporte de tu LMS.

#### El lanzamiento muestra "Configuración no encontrada"

O bien la configuración en FastComments fue eliminada, o el lanzamiento provino de un par `iss`/`client_id` que no reconocemos. Si eliminaste y volviste a registrar, indica a tu LMS que elimine y vuelva a añadir la herramienta FastComments para que obtenga el nuevo `client_id`.

#### El lanzamiento muestra "Despliegue no registrado"

Iniciaste FastComments desde un despliegue de Brightspace/Moodle/Blackboard diferente del que se usó en el primer lanzamiento. FastComments fija el `deployment_id` en el primer lanzamiento como verificación de seguridad. Para añadir un nuevo despliegue bajo el mismo cliente, contacta con el soporte - añadiremos el ID de despliegue a la configuración.

#### El lanzamiento muestra "message_type no compatible"

El LMS envió un mensaje LTI que FastComments no maneja (por ejemplo `LtiSubmissionReviewRequest`). FastComments solo soporta los flujos estándar de resource-link launch y deep-linking. Contacta si necesitas que se añada un tipo de mensaje específico.

#### El iframe no se redimensiona

La mayoría de los LMS ajustan automáticamente el tamaño de los iframes LTI. Si el tuyo no lo hace, verifica que la configuración de lanzamiento del LMS permita que la herramienta envíe eventos postMessage al frame padre. FastComments emite mensajes de redimensionamiento tanto de estilo Canvas (`lti.frameResize`) como según la especificación IMS (`org.imsglobal.lti.frameResize`).
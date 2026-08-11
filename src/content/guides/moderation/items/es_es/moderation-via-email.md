FastComments admite un resumen por correo electrónico diario, semanal o mensual para Moderadores y Administradores.

La frecuencia de esto se puede configurar <a href="" target="_blank">aquí</a>.

[app-screenshot-start url='/auth/my-account/edit-notifications?demoDigestFrequencyValue=0'; linkUrl='/auth/my-account/edit-notifications'; selector = '.content form'; alt='Formulario de Editar Notificaciones donde se configura que el resumen llegue diariamente, semanalmente o mensualmente'; title='Configurando la frecuencia del resumen' app-screenshot-end]

Mientras incluye estadísticas generales sobre sus comentarios, también enumerará los tres comentarios más recientes que necesitan revisión.

Para cada uno de dichos comentarios, se proporcionan enlaces mágicos directos para:
- Aprobar el comentario.
- Marcar el comentario como revisado y ir a la página de respuesta.
- Marcar el comentario como spam.

Estos enlaces para cada comentario lo autenticarán automáticamente y realizarán la acción desde su correo electrónico.

Además, un botón de Moderar Comentarios se encuentra en el Resumen, el cual realizará la misma autenticación y lo llevará a la página de Moderar Comentarios.

Tenga en cuenta que estos enlaces mágicos expiran después de un tiempo.

[app-screenshot-start url='/test-e2e/email/tenant-comment-digest?HOST=http%3A%2F%2Flocalhost%3A3001&stats=%7B"hasHistory"%3Atrue%2C"newCommentsCount"%3A10002%2C"hasNewCommentsIncreased"%3Atrue%2C"hasNewCommentsDecreased"%3Afalse%2C"approvedCommentsCount"%3A44%2C"hasApprovedCommentsIncreased"%3Afalse%2C"hasApprovedCommentsDecreased"%3Atrue%2C"spamCommentsCount"%3A21%2C"hasSpamCommentsIncreased"%3Afalse%2C"hasSpamCommentsDecreased"%3Atrue%2C"newUsersCount"%3A30%2C"hasNewUsersIncreased"%3Atrue%2C"hasNewUsersFalse"%3Afalse%7D&BANNER_TEXT=FastComments%20Monthly%20Digest&commentCount=100000&hasCommentsNeedsReview=true&comments=%5B%7B"commenterName"%3A"Devon%20Winrick"%2C"commentHTML"%3A"This%20is%20a%20very%20recent%20comment%20that%20needs%20approval."%2C"date"%3A1588812198540%2C"locale"%3A"en_us"%2C"avatarSrc"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o.jpg"%2C"url"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o"%7D%2C%7B"commenterName"%3A"Devon"%2C"commentHTML"%3A"This%20is%20a%20somewhat%20recent%20comment%20that%20needs%20approval."%2C"date"%3A1588812198540%2C"locale"%3A"en_us"%2C"avatarSrc"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o.jpg"%2C"url"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o"%7D%2C%7B"commenterName"%3A"Bob"%2C"commentHTML"%3A"This%20is%20a%20kind%20of%20recent%20comment%20that%20needs%20approval."%2C"date"%3A1588812198540%2C"locale"%3A"en_us"%2C"avatarSrc"%3A"https%3A%2F%2Ffastcomments.com%2Fimages%2Funknown-person.png"%2C"url"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o"%7D%5D&locale=en_us&digestEmail=%7B"tenantId"%3A"tenant-id"%2C"userId"%3A"user-id"%2C"_id"%3A"some-id"%2C"temporaryId"%3A"temporary-id"%7D&API_KEY=T0ph%20123!&rawTemporaryId=xyz'; linkUrl=false; selector = '.content'; alt='Correo electrónico de resumen mensual con estadísticas de comentarios y tres comentarios que necesitan revisión, cada uno con enlaces para aprobar, responder y marcar como spam'; title='Correo de resumen' app-screenshot-end]

#### Notification Types

FastComments envía varios tipos de correos electrónicos a Moderadores y Administradores. Si lo desea, es posible desactivar las notificaciones de `Comment Reply`, mientras sigue recibiendo notificaciones de `New Comment` eligiendo las opciones apropiadas en la página `Edit Notifications` mostrada arriba.
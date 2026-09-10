## Zaps de ejemplo

Algunos flujos de trabajo que se configuran en minutos.

**Recibe notificaciones sobre nuevos comentarios.** New Comment, then Slack "Send Channel Message" or Discord "Send Channel Message". Mapea el nombre del comentarista, el texto del comentario y la URL de la página en el mensaje. Añade el filtro de dominio para notificar a un canal diferente por sitio.

**Mantén un registro de cada comentario.** New Comment, then Google Sheets "Create Spreadsheet Row". Añade Deleted Comment como un segundo Zap que agrega una fila con el id del comentario, de modo que la hoja también sirve como registro de auditoría.

**Envía un correo electrónico al autor cuando se aprueba un comentario.** Updated Comment con un filtro de Zapier donde Approved es true, then Gmail "Send Email". Debido a que Updated Comment se dispara en cada cambio, el filtro es lo que hace que este Zap reaccione solo a aprobaciones.

**Añade a los comentaristas a tu CRM o lista de correo.** New Comment, then HubSpot "Create or Update Contact" or Mailchimp "Add or Update Subscriber" usando el correo electrónico del comentarista. Respeta tu política de privacidad y la legislación local antes de añadir a alguien a una lista de marketing.

**Crea un comentario a partir de un formulario.** Typeform or Google Forms "New Response", then FastComments Create Comment con el ID de URL de página que tu sitio usa para testimonios. Deja Approved sin marcar para revisar cada uno antes de que aparezca.

**Publica anuncios en un feed.** RSS by Zapier "New Item in Feed", then Create Feed Post con el título, contenido y enlace del elemento.

**Provee a los miembros como usuarios SSO.** Memberstack, Memberful, o tu propio webhook, then Find SSO User seguido de Create SSO User en modo "find or create".

**Escala los comentarios reportados.** Updated Comment, filtrado con un recuento de flags superior a cero, then Trello "Create Card" or Linear "Create Issue" con el id del comentario y un enlace a la página de moderación.

**Publica páginas a medida que se ponen en línea.** WordPress or Ghost "New Post", then Create Page con la URL de la publicación, de modo que la página se liste y restrinja antes del primer comentario.

**Archiva los comentarios eliminados.** Deleted Comment, then Airtable "Create Record" con el comentario completo para retención de cumplimiento.
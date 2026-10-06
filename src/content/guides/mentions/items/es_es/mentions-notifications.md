---
Los usuarios etiquetados recibirán un correo electrónico informándoles que han sido etiquetados o mencionados en un comentario.

[app-screenshot-start url='/test-e2e/email/comment-user-mention?comment=%7B"commenterName"%3A"Alexander"%2C"comment"%3A"Hey%20%40winrid%20I%20wanted%20you%20to%20see%20this."%2C"commentHTML"%3A"Hey%20<b>%40winrid<%2Fb>%20I%20wanted%20you%20to%20see%20this."%2C"date"%3A1633998787864%2C"pageTitle"%3A"Some%20Page%20Title"%7D&username=winrid&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&INTRO=Hey%20winrid%2C&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&unsubscribeLink=%7B"url"%3A"%2Fauth%2Fmy-account%2Fedit-notifications"%2C"textId"%3A"UNSUBSCRIBE_HERE"%7D&viewCommentUrl=https%3A%2F%2Fexample.com%23fast-comments-jt%3Dsome-db-id&locale=en_us&canReplyByEmail=true&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='Cuerpo del correo electrónico de notificación citando un comentario con la mención @winrid en negrita, más enlaces de ver y darse de baja'; title='Notificación de Usuario Mencionado' app-screenshot-end]

Desactivar las notificaciones evitará estos correos electrónicos, y se incluye un encabezado en cada correo para que los clientes de correo permitan al usuario darse de baja de forma fluida.

Los usuarios etiquetados también reciben una notificación dentro de la aplicación, incluso cuando no tienen una dirección de correo registrada.

Si el comentario está esperando la aprobación del moderador, el correo de mención se retiene y se envía una vez que el comentario es aprobado. No se envía si el comentario se marca como spam, se elimina o no se aprueba dentro de los 7 días.

Para ver si a un usuario mencionado se le envió un correo electrónico, o por qué no lo recibió, abra los [Registros de Comentario](/guide-moderation.html#comment-logs) del comentario desde la página Moderar Comentarios.

---
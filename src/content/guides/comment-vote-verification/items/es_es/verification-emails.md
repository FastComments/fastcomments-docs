Cuando un usuario deja un comentario o un voto, y no ha iniciado sesión, o su cuenta no está verificada, recibirá un correo electrónico pidiéndole que verifique esta acción.

Sin embargo, hacemos todo lo posible por no saturar a sus usuarios con correos electrónicos, y no enviaremos más de un correo de verificación por sesión. Consulte la sección de Sesiones para más detalles.

Por defecto, los correos de verificación de comentarios se ven de la siguiente manera:

[app-screenshot-start url='/test-e2e/email/commenter-verify-post?comment=%7B"commenterName"%3A"Alexander"%2C"comment"%3A"This%20is%20my%20comment."%2C"commentHTML"%3A"This%20is%20my%20comment."%2C"date"%3A1588812198540%7D&removedInDays=3&removedInDaysCount=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyPostUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='Cuerpo del correo de verificación por defecto citando el comentario de Alexander con un botón para confirmar la publicación'; title='Correo de verificación de comentario' app-screenshot-end]

Por defecto, los correos de verificación de votos se ven de la siguiente manera:

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='Correo por defecto pidiendo a Devon confirmar un voto, mostrando el comentario votado y un botón de confirmación'; title='Correo de verificación de voto' app-screenshot-end]

Por defecto, FastComments mostrará su logotipo y nombre en el pie de página de estos correos:

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.footer'; width = 700; alt='Parte inferior de un correo de verificación mostrando el logotipo y nombre por defecto de FastComments en el pie de página'; title='Pie de página del correo' app-screenshot-end]

Si está en los planes Flex o Pro, [El nombre del remitente, el correo electrónico y la marca se pueden personalizar](/guide-multiple-sites.html#from-name-email-logo).
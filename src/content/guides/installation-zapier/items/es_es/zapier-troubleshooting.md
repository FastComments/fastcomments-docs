## Solución de problemas

**"No tienes permiso" al conectar.** El usuario conectado no es un administrador API en la cuenta.  
Pídele al propietario de la cuenta que conceda el permiso API en la página de Usuarios, o conéctate como el propietario.

**La conexión está etiquetada con el sitio incorrecto.** La página de consentimiento conecta la cuenta con la que iniciaste  
sesión en ese momento. Desconecta en Zapier, cambia de cuenta en el panel de FastComments y vuelve a conectar.

**Los eventos dejaron de llegar.** Revisa la página de Webhooks en el panel. Una suscripción cuyo endpoint siguió  
fallando durante seis días se desactiva automáticamente y muestra la razón. Vuelve a habilitarla allí, o apaga y enciende  
el Zap nuevamente. Si la suscripción falta por completo, alguien la eliminó; apagar y encender el Zap la recrea.

**Zapier indica que la cuenta necesita reconectarse.** La conexión fue revocada desde la página de Aplicaciones Conectadas  
y el usuario que la aprobó perdió el permiso API, o la cuenta fue eliminada. Reconecta desde Zapier.

**Una acción falla con "no tiene acceso de escritura".** La conexión se aprobó con permiso de solo lectura  
y necesita ambos permisos. Reconecta y aprueba ambos permisos.

**Límites de velocidad y créditos.** Las acciones y búsquedas consumen créditos API de tu plan y están sujetas a los  
mismos límites de velocidad que la API REST. Los disparadores no consumen ninguno. Un Zap que alcanza un límite se reintenta por Zapier  
después del retraso que informa FastComments.

**El menú desplegable de Dominio está vacío.** Los dominios aparecen una vez que se configuran en la página de Dominios en el  
panel de FastComments. Deja el campo vacío para recibir eventos de todos los dominios.
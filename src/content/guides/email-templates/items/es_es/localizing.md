FastComments es una plataforma localizada. Todos nuestros widgets, correos electrónicos y notificaciones están localizados.

Localizado significa que mostramos un idioma y un formato diferentes, basados en la ubicación del usuario
y su idioma preferido. Determinamos esto a partir de la información que nos proporciona el navegador del usuario.

Podemos personalizar el texto en el correo electrónico yendo a la pestaña `Translations`, seleccionando un `Locale`
y editando el texto. El texto que se cambia respecto al valor predeterminado aparece resaltado en la interfaz. Usted puede
cambiar entre locales y guardar al final, sin perder los cambios.

El texto localizado se accede a través del objeto `TEXT`, por ejemplo: `<%= TEXT.INTRO %>`.

### Nota sobre SSO

Para las integraciones SSO, si no se especifica `locale`, este se actualizará cada vez que el usuario
acceda al widget de comentarios con un locale diferente. Esto significa que su preferencia de idioma
se actualizará automáticamente, y los correos electrónicos futuros se enviarán en ese locale.

Esto también se puede establecer manualmente proporcionando `locale` en el payload de SSO.
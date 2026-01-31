Aquí hay algunos síntomas que vemos con frecuencia y soluciones comunes.

### "This is a demo" Message

Esto se muestra cuando has copiado el código del widget desde nuestra página principal, que usa nuestro tenant de demostración. Para usar tu tenant, copia el código del widget desde [here](https://fastcomments.com/auth/my-account/get-acct-code).

### "FastComments cannot load on this domain" Error

FastComments necesita saber qué dominios te pertenecen para autenticar las solicitudes asociadas con tu cuenta. [Consulta nuestra documentación](/guide-multiple-sites.html#add-domains-to-account) para ver cómo resolver este error (simplemente añade el subdominio + dominio exacto a tu cuenta).

Ten en cuenta que esto solo debería ocurrir una vez finalizado el periodo de prueba. Durante el periodo de prueba, cualquier solicitud desde dominios nuevos se añadirá automáticamente a tu cuenta.

### Migrated Comments Not Showing for Custom Installations

Normalmente esto ocurre cuando los comentarios importados están ligados a un `Page ID`, y estás pasando una URL (o ningún valor, en cuyo caso toma por defecto la URL de la página).

Puedes depurar esto [exportando tus comentarios](https://fastcomments.com/auth/my-account/manage-data/export) y viendo la columna `URL ID` (actualmente la Columna `B`).

Asegúrate de que los valores que ves en la columna `URL ID` sean los mismos valores que estás pasando a la configuración del widget como el parámetro `urlId`.

Para una explicación más detallada, intenta leer nuestra documentación [How Comments are Tied to Pages and Articles](/guide-customizations-and-configuration.html#url-id).

Si todo falla, [contáctanos](https://fastcomments.com/auth/my-account/help).

### Comment Widget Not Showing

Si el widget de comentarios no se muestra, revisa la consola de desarrollador de Chrome para ver errores.

Para la mayoría de las configuraciones incorrectas, el widget de comentarios al menos mostrará un error en la página si puede cargarse. No ver nada suele ser indicativo de un error de scripting.

### Desired Configuration Not Working as Expected

Prueba nuestra [extensión de Chrome](https://chrome.google.com/webstore/detail/fastcomments-debugger/cadggdemhfkjjghkdbfhonoccnplffjj?hl=en-US) para ver qué configuración se le está pasando al widget de comentarios. Si todo falla, toma una captura de pantalla de lo que dice la extensión de Chrome y [contáctanos](https://fastcomments.com/auth/my-account/help).

### Comments Missing on Same URL With Different Hash Bang

Por defecto, FastComments usará la URL de la página para el "bucket" donde se almacenan los comentarios. Si tus URLs incluyen `#hashbangs`, y estos `#hashbangs` no deberían ser parte del identificador que identifica un hilo de comentarios, podemos simplemente ignorar el valor del hash bang, por ejemplo:

[inline-code-attrs-start title = 'Ejemplo: Ignorar Hash Bangs'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.fcConfigs = [{
    target: '#fastcomments-widget',
    tenantId: "demo",
    url: location.href.replace(location.hash, ''),
    urlId: location.href.replace(location.hash, '')
}];
</script>
[inline-code-end]

Ten en cuenta que después de hacer este cambio, se tendrá que realizar una migración para los comentarios existentes. [Para eso, contáctanos.](https://fastcomments.com/auth/my-account/help)

### URL Query Parameters Affecting Widget

Por defecto, FastComments usará la URL de la página para el "bucket" donde se almacenan los comentarios. Si tus URLs incluyen parámetros de consulta que no deberían ser parte del identificador que identifica un hilo de comentarios, podemos simplemente ignorarlos, por ejemplo:

[inline-code-attrs-start title = 'Ignorar parámetros de consulta'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.fcConfigs = [{
    target: '#fastcomments-widget',
    tenantId: "demo",
    url: location.protocol + '//' + location.host + location.pathname,
    urlId: location.pathname
}];
</script>
[inline-code-end]

Ten en cuenta que después de hacer este cambio, se tendrá que realizar una migración para los comentarios existentes. [Para eso, contáctanos.](https://fastcomments.com/auth/my-account/help)

### Not Receiving Emails

En FastComments, dedicamos mucho trabajo a asegurar que la entrega de correos electrónicos sea lo más fiable posible. Sin embargo, algunos proveedores de correo son notoriamente difíciles de entregar de forma fiable. Revisa tu carpeta de spam para mensajes de fastcomments.com.

Si [nos contactas](https://fastcomments.com/auth/my-account/help), normalmente podemos proporcionar más información sobre por qué puede que no estés recibiendo correos nuestros.
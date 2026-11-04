Aquí hay algunos síntomas que encontramos con frecuencia, junto con sus soluciones habituales.

### Mensaje "This is a demo"

Esto se muestra cuando has copiado el código del widget desde nuestra página de inicio, que utiliza nuestro tenant de demostración. Para usar tu tenant, copia el código del widget desde [aquí](https://fastcomments.com/auth/my-account/get-acct-code).

### Error "FastComments cannot load on this domain"

FastComments necesita saber qué dominios te pertenecen para autenticar las solicitudes asociadas con tu cuenta. [Consulta nuestra documentación](/guide-multiple-sites.html#add-domains-to-account) para ver cómo resolver este error (simplemente añade el subdominio + dominio exactos a tu cuenta).

Ten en cuenta que esto solo debería ocurrir después de que termine el período de prueba. Durante el período de prueba, cualquier solicitud de nuevos dominios se añadirá automáticamente a tu cuenta.

### Comentarios Migrados que No Aparecen en Instalaciones Personalizadas

Normalmente esto sucede cuando los comentarios importados están vinculados a un `Page ID`, y estás pasando una URL (o ningún valor, en cuyo caso se usa por defecto la URL de la página).

Puedes depurar esto [exportando tus comentarios](https://fastcomments.com/auth/my-account/manage-data/export) y viendo la columna `URL ID` (actualmente la columna `B`).

Asegúrate de que los valores que ves en la columna `URL ID` sean los mismos valores que estás pasando a la configuración del widget como parámetro `urlId`.

Para una explicación más detallada, intenta leer nuestra [documentación sobre cómo los comentarios están vinculados a páginas y artículos](/guide-customizations-and-configuration.html#url-id).

Si nada funciona, [contáctanos](https://fastcomments.com/auth/my-account/help).

### El Widget de Comentarios No Aparece

Si el widget de comentarios no aparece, revisa la consola de desarrollador de Chrome en busca de errores.

En la mayoría de los casos de mala configuración, el widget de comentarios mostrará al menos un error en la página si es capaz de cargarse. No ver nada suele ser una indicación de un error de scripting.

### La Configuración Deseada No Funciona Como Se Esperaba

Prueba nuestra [extensión de Chrome](https://chromewebstore.google.com/detail/fastcomments-debugger/cadggdemhfkjjghkdbfhonoccnplffjj) para ver qué configuración se está pasando al widget de comentarios. Si nada funciona, toma una captura de pantalla de lo que dice la extensión de Chrome y [contáctanos](https://fastcomments.com/auth/my-account/help).

### Comentarios Faltantes en la Misma URL con Diferente Hash Bang

Por defecto, FastComments utilizará la URL de la página como el "bucket" donde se almacenan los comentarios. Si tus URLs incluyen `#hashbangs`, y estos `#hashbangs` no deberían formar parte del identificador que identifica un hilo de comentarios, podemos simplemente ignorar el valor del hash bang, por ejemplo:

[inline-code-attrs-start title = 'Ejemplo de Ignorar Hash Bangs'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

### Parámetros de Consulta de la URL que Afectan al Widget

Por defecto, FastComments utilizará la URL de la página como el "bucket" donde se almacenan los comentarios. Si tus URLs incluyen parámetros de consulta que no deberían formar parte del identificador que identifica un hilo de comentarios, podemos simplemente ignorarlos, por ejemplo:

[inline-code-attrs-start title = 'Ignorar Parámetros de Consulta'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

### No Se Reciben Correos Electrónicos

En FastComments, dedicamos mucho trabajo a garantizar que la entrega de nuestros correos electrónicos sea lo más confiable posible. Sin embargo, algunos proveedores de correo son notoriamente difíciles para una entrega confiable. Revisa tu carpeta de spam en busca de mensajes de fastcomments.com.

Si nos [contactas](https://fastcomments.com/auth/my-account/help), normalmente podemos proporcionar más información sobre por qué es posible que no estés viendo correos de nuestra parte.
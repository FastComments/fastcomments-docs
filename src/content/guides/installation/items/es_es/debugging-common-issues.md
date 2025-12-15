Aquí hay algunos síntomas que encontramos frecuentemente y soluciones comunes.

### Mensaje "This is a demo"

Esto se muestra cuando ha copiado el código del widget de nuestra página de inicio, que usa nuestro inquilino de demostración. Para usar su inquilino, copie el código del widget desde [aquí](https://fastcomments.com/auth/my-account/get-acct-code).

### Error "FastComments cannot load on this domain"

FastComments necesita saber qué dominios le pertenecen para autenticar las solicitudes asociadas con su cuenta. [Consulte nuestra documentación](/guide-multiple-sites.html#add-domains-to-account) para ver cómo resolver este error (simplemente agregue el subdominio exacto + dominio a su cuenta).

Tenga en cuenta que esto solo debería ocurrir después de que termine el período de prueba. Durante el período de prueba, cualquier solicitud de nuevos dominios se agregará automáticamente a su cuenta.

### Los comentarios migrados no se muestran en instalaciones personalizadas

Generalmente, esto sucede cuando los comentarios importados están vinculados a un `Page ID` y usted está pasando una URL (o ningún valor, en cuyo caso se establece por defecto a la URL de la página).

Puede depurar esto [exportando sus comentarios](https://fastcomments.com/auth/my-account/manage-data/export) y viendo la columna `URL ID` (actualmente la columna `B`).

Asegúrese de que los valores que ve en la columna `URL ID` sean los mismos valores que está pasando a la configuración del widget como el parámetro `urlId`.

Para más explicaciones, intente leer nuestra [documentación sobre cómo los comentarios están vinculados a páginas y artículos](/guide-customizations-and-configuration.html#url-id).

Si todo lo demás falla, [contáctenos](https://fastcomments.com/auth/my-account/help).

### El widget de comentarios no se muestra

Si el widget de comentarios no se muestra, verifique la consola de desarrollador de Chrome para errores.

Para la mayoría de las configuraciones incorrectas, el widget de comentarios al menos mostrará un error en la página si puede cargarse. No ver nada generalmente es una indicación de un error de script.

### La configuración deseada no funciona como se esperaba

Pruebe nuestra [extensión de Chrome](https://chrome.google.com/webstore/detail/fastcomments-debugger/cadggdemhfkjjghkdbfhonoccnplffjj?hl=en-US) para ver qué configuración se está pasando al widget de comentarios. Si todo lo demás falla, tome una captura de pantalla de lo que dice la extensión de Chrome y [contáctenos](https://fastcomments.com/auth/my-account/help).

### Comentarios faltantes en la misma URL con diferente hash bang

Por defecto, FastComments usará la URL de la página para el "bucket" donde se almacenan los comentarios. Si sus URLs incluyen `#hashbangs`, y estos `#hashbangs` no deberían ser parte del identificador que identifica un hilo de comentarios, simplemente podemos ignorar el valor del hash bang, por ejemplo:

[inline-code-attrs-start title = 'Ignore Hash Bangs Example'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
    tenantId: "demo",
    url: location.href.replace(location.hash, ''),
    urlId: location.href.replace(location.hash, '')
});
</script>
[inline-code-end]

Tenga en cuenta que después de hacer este cambio, se tendrá que realizar una migración para los comentarios existentes. [Para eso, contáctenos.](https://fastcomments.com/auth/my-account/help)

### Los parámetros de consulta de URL afectan al widget

Por defecto, FastComments usará la URL de la página para el "bucket" donde se almacenan los comentarios. Si sus URLs incluyen parámetros de consulta que no deberían ser parte del identificador que identifica un hilo de comentarios, simplemente podemos ignorarlos, por ejemplo:

[inline-code-attrs-start title = 'Ignore Query Parameters'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
    tenantId: "demo",
    url: location.protocol + '//' + location.host + location.pathname,
    urlId: location.pathname
});
</script>
[inline-code-end]

Tenga en cuenta que después de hacer este cambio, se tendrá que realizar una migración para los comentarios existentes. [Para eso, contáctenos.](https://fastcomments.com/auth/my-account/help)

### No recibe correos electrónicos

En FastComments, ponemos mucho trabajo para asegurar que nuestra entrega de correos electrónicos sea lo más confiable posible. Sin embargo, algunos proveedores de correo electrónico son notoriamente difíciles de alcanzar de manera confiable. Revise su carpeta de spam para mensajes de fastcomments.com.

Si [nos contacta](https://fastcomments.com/auth/my-account/help), generalmente podemos proporcionar más información sobre por qué puede no estar viendo correos electrónicos de nuestra parte.

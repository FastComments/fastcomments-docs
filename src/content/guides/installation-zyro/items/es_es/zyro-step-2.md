Ahora añadamos el código de nuestro widget.

Copia el código a continuación. Asegúrate de haber iniciado sesión en [fastcomments.com](https://fastcomments.com) 
y recarga esta página si no es así, para que el código se complete automáticamente con la información de tu cuenta, de lo contrario mostrará el código de demostración.

Ahora vamos a copiar el código:

[inline-code-attrs-start title = 'Código de Zyro Comments'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo",
        pageTitle: window.parent.document.title,
        urlId: window.parent.location.href,
        url: window.parent.location.href
    }];
</script>
[inline-code-end]

Now let's go back to our site builder and click `Enter code`:

<div class="screenshot white-bg">
    <div class="title">Ingresar código</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-2.png" alt="Ingresar código" />
</div>

### ¡Nota!

Es importante que utilices el código anterior y no los fragmentos de código de otra documentación, ya que este fragmento ha sido creado específicamente
para Zyro.

Ahora deberías tener algo como lo siguiente, que aparece en blanco. Esto es normal. Mueve el ratón sobre el área
donde debería aparecer el widget:

<div class="screenshot white-bg">
    <div class="title">Widget de código agregado</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-3.png" alt="Widget de código agregado" />
</div>

Ahora arrastra el widget hasta el tamaño deseado, lo verás aparecer:

<div class="screenshot white-bg">
    <div class="title">Ajustar tamaño</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-4.png" alt="Ajustar tamaño" />
</div>

...y ahora ¡previsualiza y guarda!
Ahora vamos a añadir el código de nuestro widget.

Copia el código a continuación. Asegúrate de haber iniciado sesión en [fastcomments.com](https://fastcomments.com) 
y recarga esta página si no lo estás, para que el código se rellene con la información de tu cuenta; de lo contrario mostrará el código de demostración.

Ahora copiemos el código:

[inline-code-attrs-start title = 'Código de comentarios de Hostinger'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Ahora volvamos al constructor del sitio y haz clic en `Enter code`:

<div class="screenshot white-bg">
    <div class="title">Introducir código</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-2.png" alt="Introducir código" />
</div>

### ¡Nota!

Es importante que uses el código anterior y no los fragmentos de código de otra documentación, ya que este fragmento ha sido creado específicamente
para Hostinger.

Deberías ver ahora algo similar a lo siguiente, que aparece en blanco. Esto es normal. Mueve el ratón sobre el área
donde debería aparecer el widget:

<div class="screenshot white-bg">
    <div class="title">Widget de código añadido</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-3.png" alt="Widget de código añadido" />
</div>

Ahora arrastra el widget hasta el tamaño deseado; verás que aparece:

<div class="screenshot white-bg">
    <div class="title">Ajusta su tamaño</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-4.png" alt="Ajusta su tamaño" />
</div>

...¡y ahora previsualiza y guarda!

---
Ahora agreguemos el código de nuestro widget.

Copia el código a continuación. Asegúrate de haber iniciado sesión en [fastcomments.com](https://fastcomments.com) 
y recargar esta página si no lo has hecho, de modo que el código se precomplete con la información de tu cuenta; de lo contrario mostrará el código de demostración.

Ahora copiemos el código:

[inline-code-attrs-start title = 'Código de comentarios de Zyro'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: "demo",
        pageTitle: window.parent.document.title,
        urlId: window.parent.location.href,
        url: window.parent.location.href
    });
</script>
[inline-code-end]

Ahora volvamos al constructor de sitios y haz clic en `Enter code`:

<div class="screenshot white-bg">
    <div class="title">Insertar código</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-2.png" alt="Insertar código" />
</div>

### ¡Nota!

Es importante que uses el código anterior y no los fragmentos de código de otra documentación, ya que este fragmento ha sido creado específicamente
para Zyro.

Ahora deberías tener algo parecido a lo siguiente, que aparece en blanco. Esto es normal. Mueve el cursor sobre el área
donde debería estar el widget:

<div class="screenshot white-bg">
    <div class="title">Widget de código añadido</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-3.png" alt="Widget de código añadido" />
</div>

Ahora arrastra el widget al tamaño deseado, verás que aparece:

<div class="screenshot white-bg">
    <div class="title">Cambiar tamaño</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-4.png" alt="Cambiar tamaño" />
</div>

...y ahora previsualiza y guarda!

---
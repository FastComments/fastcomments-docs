---
Ahora vamos a copiar nuestro código. Si el fragmento de código dice `tenantId: "demo"` en la línea 6 entonces debes iniciar sesión en tu cuenta de FastComments
y luego actualizar esta página para que el fragmento copiado tenga tu account id.

[inline-code-attrs-start title = 'Fragmento de Systeme.io'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: "demo"
    });
</script>
[inline-code-end]

Ahora pégalo en el editor y haz clic en guardar:

<div class="screenshot white-bg">
    <div class="title">Añadir el código de FastComments</div>
    <img class="screenshot-image" src="/images/installation-guides/systeme-add-code.png" alt="Añadir el código de FastComments" />
</div>

... then save your site. That's it!

---
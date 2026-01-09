---
FastComments también admite el widget Page Reacts (también conocido como botón Me gusta flotante) para Hostinger.

¡Puedes verlo en acción en la esquina inferior derecha de esta página!

### ¡Nota!

Estas instrucciones son para el Site Builder de Hostinger. Si estás usando Hostinger *WordPress*, simplemente toma el código a continuación y añádelo a tu sitio WordPress
usando [WPCode](https://wordpress.org/plugins/insert-headers-and-footers/), que es un complemento gratuito y sencillo para añadir pequeños fragmentos de código a tu sitio.

1. Primero, toma el código:

[inline-code-attrs-start title = 'Código del botón Me gusta flotante de Hostinger'; type = 'bash'; useDemoTenant = false; isFunctional = false; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-page-likes-floating.min.js?v=2" async></script>
<div id="fastcomments-page-likes-floating"></div>
<script>
    (function () {
        function tryLoad() {
            if (typeof window.FastCommentsEmbedPageLikesFloating === 'function') {
                window.FastCommentsEmbedPageLikesFloating(document.getElementById('fastcomments-page-likes-floating'), {
                    tenantId: "demo"
                });
            } else {
                setTimeout(tryLoad, 50);
            }
        }

        tryLoad();
    })();
</script>
[inline-code-end]

2. Luego, en Hostinger, abre el Site Builder.
3. Ve a Ajustes del sitio web en la parte inferior izquierda.
4. Selecciona Integraciones.
5. Añade el nuevo código al *final* del campo `Custom code`, y publica tu sitio.
6. No verás el widget en el modo de vista previa, pero aparecerá en la versión publicada del sitio.

---
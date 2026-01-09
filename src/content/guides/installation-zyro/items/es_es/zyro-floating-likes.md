---
FastComments también es compatible con el widget Page Reacts (también conocido como botón Me gusta flotante) para Zyro.

¡Puedes verlo en acción en la esquina inferior derecha de esta página!

1. Primero, obtén el código:

[inline-code-attrs-start title = 'Código del botón Me gusta flotante de Zyro'; type = 'bash'; useDemoTenant = false; isFunctional = false; type = 'html';  inline-code-attrs-end]
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

2. Luego, en Zyro, abre el creador de sitios.
3. Ve a Configuración del sitio web en la esquina inferior izquierda.
4. Selecciona Integraciones.
5. Añade el nuevo código al *final* del campo `Custom code`, y publica tu sitio.
6. No verás el widget en el modo de vista previa, pero aparecerá en la versión publicada del sitio.

---
---
A continuación se muestra el código Vanilla JS para instalar el widget de resumen. La biblioteca de React también incluye este widget.

[inline-code-attrs-start title = 'Instalación del widget de resumen'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-reviews-summary.min.js"></script>
<div id="summary-widget"></div>
<script>
    window.FastCommentsReviewsSummaryWidget(document.getElementById('summary-widget'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

El widget encontrará automáticamente las preguntas que debe mostrar en el resumen según la configuración correspondiente del widget para esa página/sitio.

Si necesita el widget en alguna de nuestras otras bibliotecas que no lo incluya, abra un ticket de soporte para que sepamos que debemos añadirlo.

---
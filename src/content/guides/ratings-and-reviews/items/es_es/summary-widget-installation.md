A continuación se muestra el código Vanilla JS para instalar el Summary Widget. La librería React también tiene este widget.

[inline-code-attrs-start title = 'Instalación del Summary Widget'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-reviews-summary.min.js"></script>
<div id="summary-widget"></div>
<script>
    window.FastCommentsReviewsSummaryWidget(document.getElementById('summary-widget'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

El widget encontrará automáticamente las preguntas que se mostrarán en el resumen según la configuración del widget correspondiente para esa página/sitio.

### Filtrar Comentarios por Calificación

Cuando el widget de comentarios de la misma página también está en la página, y está configurado para mostrar la respuesta de cada comentarista en su comentario, las filas del desglose
se vuelven clicables. Al hacer clic en una fila, por ejemplo "5", se filtra el widget de comentarios a los comentarios que dieron esa respuesta. Al hacer clic en la fila de nuevo, o en la X del chip "Mostrando solo"
en el widget de comentarios, se muestran todos los comentarios de nuevo. Esto no requiere configuración: el Summary Widget encuentra el widget de comentarios con el mismo `tenantId` y `urlId`.

Esto funciona para preguntas de estrellas, pulgares, emojis, escala y de opción única. El desglose que el widget de comentarios muestra sobre el cuadro de comentarios se puede hacer clic de la misma manera.

Si necesita el widget en una de nuestras otras librerías que no lo tiene, abra un ticket de soporte para que sepamos que debemos añadirlo.
---
Aquí hay una pequeña demostración del widget de resumen:

<script src="https://cdn.fastcomments.com/js/embed-reviews-summary.min.js"></script>
<div id="summary-widget"></div>
<script>
    window.FastCommentsReviewsSummaryWidget(document.getElementById('summary-widget'), {
        tenantId: 'L177BUDVvSe',
        urlId: window.location.origin + window.location.pathname // eliminar los parámetros de consulta
    });
</script>

### Caché

Tenga en cuenta que los resúmenes se almacenan en caché durante 30 segundos, o cinco minutos si hay un gran número de reseñas. Debido a esto, su reseña puede no aparecer de inmediato en el resumen, pero esto
nos permite reducir el coste de servir el widget de resumen.

---
The widget Discusiones recientes muestra las páginas de tu sitio que tienen la actividad de comentarios más reciente. Cada entrada muestra el título de la página, la fecha de la última actividad y el recuento total de comentarios. Detecta automáticamente fondos oscuros y ajusta su estilo en consecuencia.

## Instalación básica

[inline-code-attrs-start title = 'Instalación del widget de discusiones recientes'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-recent-discussions-v2.min.js"></script>
<div id="fastcomments-widget-recent-discussions"></div>
<script>
    FastCommentsRecentDiscussionsV2(document.getElementById('fastcomments-widget-recent-discussions'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

## Opciones de configuración

La función `FastCommentsRecentDiscussionsV2` acepta las siguientes opciones de configuración:

- **tenantId** (required): Tu ID de tenant de FastComments
- **count** (optional): Número de páginas a mostrar. El valor predeterminado es `20`, máximo `100`
- **hasDarkBackground** (optional): Forzar el estilo de modo oscuro. Se detecta automáticamente a partir del fondo de la página si no se establece

## Ejemplos avanzados

### Recuento personalizado

[inline-code-attrs-start title = 'Discusiones recientes con recuento personalizado'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-recent-discussions-v2.min.js"></script>
<div id="fastcomments-widget-recent-discussions"></div>
<script>
    FastCommentsRecentDiscussionsV2(document.getElementById('fastcomments-widget-recent-discussions'), {
        tenantId: 'demo',
        count: 5
    });
</script>
[inline-code-end]

### Forzar modo oscuro

[inline-code-attrs-start title = 'Discusiones recientes con modo oscuro'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-recent-discussions-v2.min.js"></script>
<div id="fastcomments-widget-recent-discussions"></div>
<script>
    FastCommentsRecentDiscussionsV2(document.getElementById('fastcomments-widget-recent-discussions'), {
        tenantId: 'demo',
        hasDarkBackground: true
    });
</script>
[inline-code-end]

---
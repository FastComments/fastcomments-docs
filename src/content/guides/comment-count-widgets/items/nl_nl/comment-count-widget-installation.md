De Comment Count Widget is ontworpen voor het weergeven van het aantal reacties van een enkele pagina. Deze is lichtgewicht en biedt realtime updates indien geconfigureerd.

## Basisinstallatie

[inline-code-attrs-start title = 'Installatie Comment Count Widget'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-comment-count.min.js"></script>
<div id="comment-count"></div>
<script>
    window.FastCommentsCommentCount(document.getElementById('comment-count'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

## Configuratieopties

De functie `FastCommentsCommentCount` accepteert de volgende configuratieopties:

- **tenantId** (vereist): Je FastComments tenant ID
- **urlId** (optioneel): De pagina-identificatie. Standaard wordt `window.location.href` gebruikt indien niet opgegeven
- **numberOnly** (optioneel): Indien `true`, wordt alleen het nummer weergegeven zonder tekst. Standaard is `false`
- **isLive** (optioneel): Indien `true`, wordt het aantal automatisch bijgewerkt. Standaard is `false`

## Geavanceerde voorbeelden

### Aangepaste URL ID

[inline-code-attrs-start title = 'Comment Count met aangepaste URL ID'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-comment-count.min.js"></script>
<div id="comment-count-custom"></div>
<script>
    window.FastCommentsCommentCount(document.getElementById('comment-count-custom'), {
        tenantId: 'demo',
        urlId: 'my-custom-page-id'
    });
</script>
[inline-code-end]

### Alleen nummerweergave

[inline-code-attrs-start title = 'Comment Count alleen nummer'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-comment-count.min.js"></script>
<div id="comment-count-number"></div>
<script>
    window.FastCommentsCommentCount(document.getElementById('comment-count-number'), {
        tenantId: 'demo',
        numberOnly: true
    });
</script>
[inline-code-end]

### Live updates

[inline-code-attrs-start title = 'Live updates Comment Count'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-comment-count.min.js"></script>
<div id="comment-count-live"></div>
<script>
    window.FastCommentsCommentCount(document.getElementById('comment-count-live'), {
        tenantId: 'demo',
        isLive: true
    });
</script>
[inline-code-end]

## Widget-methoden

De widget retourneert een object met de volgende methoden:

- **destroy()**: Verwijdert de widget en ruimt alle timers op
- **update(config)**: Werkt de widget bij met een nieuwe configuratie

### Voorbeeld van gebruik

[inline-code-attrs-start title = 'Voorbeeld widget-methoden'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-comment-count.min.js"></script>
<div id="comment-count-methods"></div>
<script>
    const widget = window.FastCommentsCommentCount(document.getElementById('comment-count-methods'), {
        tenantId: 'demo'
    });
    
    // Werk de widget bij om het aantal van een andere pagina te tonen
    setTimeout(() => {
        widget.update({
            tenantId: 'demo',
            urlId: 'different-page-id'
        });
    }, 5000);
    
    // Vernietig de widget na 10 seconden
    setTimeout(() => {
        widget.destroy();
    }, 10000);
</script>
[inline-code-end]

## Styling

De widget rendert platte HTML met het aantal reacties en wordt geleverd met minimale styling. Je kunt het uiterlijk aanpassen met CSS:

[inline-code-attrs-start title = 'Aangepaste styling'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<style>
    .comment-count-styled {
        background: #f0f0f0;
        padding: 5px 10px;
        border-radius: 15px;
        font-size: 14px;
        color: #666;
        display: inline-block;
    }
</style>
<script src="https://cdn.fastcomments.com/js/widget-comment-count.min.js"></script>
<div id="comment-count-styled" class="comment-count-styled"></div>
<script>
    window.FastCommentsCommentCount(document.getElementById('comment-count-styled'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]
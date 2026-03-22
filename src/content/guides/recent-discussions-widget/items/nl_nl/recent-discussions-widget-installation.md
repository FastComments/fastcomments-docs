De Recent Discussions-widget toont pagina's op uw site met de meest recente commentaaractiviteit. Elke vermelding toont de paginatitel, datum van de laatste activiteit en het totale aantal reacties. De widget detecteert automatisch donkere achtergronden en past de opmaak dienovereenkomstig aan.

## Basisinstallatie

[inline-code-attrs-start title = 'Installatie van de Recent Discussions-widget'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-recent-discussions-v2.min.js"></script>
<div id="fastcomments-widget-recent-discussions"></div>
<script>
    FastCommentsRecentDiscussionsV2(document.getElementById('fastcomments-widget-recent-discussions'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

## Configuratie-opties

De functie `FastCommentsRecentDiscussionsV2` accepteert de volgende configuratieopties:

- **tenantId** (verplicht): Uw FastComments tenant ID
- **count** (optioneel): Aantal pagina's om te tonen. Standaard is `20`, maximaal `100`
- **hasDarkBackground** (optioneel): Forceer styling voor donkere modus. Wordt automatisch gedetecteerd op basis van de paginachtergrond als niet ingesteld

## Geavanceerde voorbeelden

### Aangepast aantal

[inline-code-attrs-start title = 'Recent Discussions met aangepast aantal'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

### Forceer donkere modus

[inline-code-attrs-start title = 'Recent Discussions met donkere modus'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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
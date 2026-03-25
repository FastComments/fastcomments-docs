De Most Discussed Pages Widget toont een gerangschikte lijst van uw meest besproken pagina's. Het bevat een koptekst, genummerde rangnummers, aantal reacties met pictogrammen, datums van laatste activiteit en automatische detectie van donkere modus.

## Basisinstallatie

[inline-code-attrs-start title = 'Installatie van de widget Meest besproken pagina\'s'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-top-pages-v2.min.js"></script>
<div id="fastcomments-widget-top-pages"></div>
<script>
    FastCommentsTopPagesV2(document.getElementById('fastcomments-widget-top-pages'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

## Configuratie-opties

- **tenantId** (vereist): Uw FastComments tenant-ID
- **hasDarkBackground** (optioneel): Forceer donkere modus-styling. Wordt automatisch gedetecteerd op basis van de achtergrond van de pagina als dit niet is ingesteld

## Widgetstructuur

De widget rendert met de volgende HTML-structuur:

[inline-code-attrs-start title = 'HTML-structuur van de widget Meest besproken pagina\'s'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div class="fc-tp2">
    <div class="fc-tp2-heading">Most Discussed Pages</div>
    <div class="fc-tp2-list">
        <div class="fc-tp2-item">
            <div class="fc-tp2-rank">1</div>
            <div class="fc-tp2-detail">
                <a class="fc-tp2-title" href="...">Page Title</a>
                <span class="fc-tp2-activity">Last activity Mar 21, 2026</span>
            </div>
            <div class="fc-tp2-count">42</div>
        </div>
    </div>
</div>
[inline-code-end]

## Standaard CSS-referentie

[inline-code-attrs-start title = 'Standaard-CSS van de widget Meest besproken pagina\'s'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
.fc-tp2 {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen, Ubuntu, Cantarell, "Helvetica Neue", sans-serif;
    text-align: left;
    line-height: 1.5;
    color: #1a1a1a;
    border: 1px solid #e0e0e0;
    border-radius: 12px;
    padding: 20px;
    background: #fff;
}
.fc-tp2-heading { font-size: 16px; font-weight: 700; margin-bottom: 14px; padding-bottom: 12px; border-bottom: 1px solid #eee; }
.fc-tp2-item { display: flex; align-items: center; gap: 12px; padding: 10px 0; border-bottom: 1px solid #f0f0f0; }
.fc-tp2-item:last-child { border-bottom: none; }
.fc-tp2-rank { width: 26px; height: 26px; display: flex; align-items: center; justify-content: center; border-radius: 50%; font-size: 11px; font-weight: 700; background: #f0f0f0; color: #888; }
.fc-tp2-title { font-size: 13px; font-weight: 500; color: #1a1a1a; text-decoration: none; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.fc-tp2-activity { font-size: 11px; color: #999; }
.fc-tp2-count { font-size: 12px; font-weight: 600; color: #666; }
[inline-code-end]

## Aanpassingsvoorbeelden

### Rangbadges verwijderen

[inline-code-attrs-start title = 'Rangbadges verwijderen'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
.fc-tp2-rank {
    display: none !important;
}
[inline-code-end]

### Verwijder de rand van de container

[inline-code-attrs-start title = 'Rand van container verwijderen'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
.fc-tp2 {
    border: none !important;
    box-shadow: none !important;
}
[inline-code-end]

---
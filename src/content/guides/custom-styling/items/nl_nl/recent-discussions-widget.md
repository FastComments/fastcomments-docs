De Recent Discussions Widget toont een lijst met pagina's, gesorteerd op de meest recente commentaaractiviteit. Deze bevat een kop, datums van de laatste activiteit, aantallen opmerkingen met pictogrammen en automatische detectie van de donkere modus.

## Basisinstallatie

[inline-code-attrs-start title = 'Styling-instelling Recent Discussions Widget'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-recent-discussions-v2.min.js"></script>
<div id="fastcomments-widget-recent-discussions"></div>
<script>
    FastCommentsRecentDiscussionsV2(document.getElementById('fastcomments-widget-recent-discussions'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

## Configuratieopties

- **tenantId** (vereist): Uw FastComments tenant ID
- **count** (optioneel): Aantal weer te geven pagina's. Standaard is `20`, maximaal `100`
- **hasDarkBackground** (optioneel): Forceer styling voor de donkere modus. Wordt automatisch gedetecteerd op basis van de pagina-achtergrond indien niet ingesteld

## Widgetstructuur

De widget wordt weergegeven met de volgende HTML-structuur:

[inline-code-attrs-start title = 'HTML-structuur Recent Discussions Widget'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div class="fc-rd2">
    <div class="fc-rd2-heading">Recent Discussions</div>
    <div class="fc-rd2-list">
        <div class="fc-rd2-item">
            <div class="fc-rd2-detail">
                <a class="fc-rd2-title" href="...">Page Title</a>
                <span class="fc-rd2-activity">Last activity Mar 21, 2026</span>
            </div>
            <div class="fc-rd2-count">42</div>
        </div>
    </div>
</div>
[inline-code-end]

## Standaard CSS-referentie

[inline-code-attrs-start title = 'Standaard CSS Recent Discussions Widget'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
.fc-rd2 {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen, Ubuntu, Cantarell, "Helvetica Neue", sans-serif;
    text-align: left;
    line-height: 1.5;
    color: #1a1a1a;
    border: 1px solid #e0e0e0;
    border-radius: 12px;
    padding: 20px;
    background: #fff;
}
.fc-rd2-heading { font-size: 16px; font-weight: 700; margin-bottom: 14px; padding-bottom: 12px; border-bottom: 1px solid #eee; }
.fc-rd2-item { display: flex; align-items: center; gap: 12px; padding: 10px 0; border-bottom: 1px solid #f0f0f0; }
.fc-rd2-item:last-child { border-bottom: none; }
.fc-rd2-title { font-size: 13px; font-weight: 500; color: #1a1a1a; text-decoration: none; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.fc-rd2-activity { font-size: 11px; color: #999; }
.fc-rd2-count { font-size: 12px; font-weight: 600; color: #666; }
[inline-code-end]

## Aanpassingsvoorbeelden

### Verwijder de rand van de container

[inline-code-attrs-start title = 'Rand van container verwijderen'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
.fc-rd2 {
    border: none !important;
    box-shadow: none !important;
}
[inline-code-end]

### Aangepaste linkkleur

[inline-code-attrs-start title = 'Aangepaste linkkleur'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
a.fc-rd2-title:hover {
    color: #e63946 !important;
}
[inline-code-end]
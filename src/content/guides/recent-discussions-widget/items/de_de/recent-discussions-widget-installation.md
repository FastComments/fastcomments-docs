Das Recent Discussions Widget zeigt Seiten auf Ihrer Website, die die jüngste Kommentaraktivität aufweisen. Jeder Eintrag zeigt den Seitentitel, das Datum der letzten Aktivität und die Gesamtanzahl der Kommentare an. Es erkennt automatisch dunkle Hintergründe und passt sein Styling entsprechend an.

## Grundinstallation

[inline-code-attrs-start title = 'Installation des Recent Discussions Widgets'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-recent-discussions-v2.min.js"></script>
<div id="fastcomments-widget-recent-discussions"></div>
<script>
    FastCommentsRecentDiscussionsV2(document.getElementById('fastcomments-widget-recent-discussions'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

## Konfigurationsoptionen

Die Funktion `FastCommentsRecentDiscussionsV2` akzeptiert die folgenden Konfigurationsoptionen:

- **tenantId** (erforderlich): Ihre FastComments tenant ID
- **count** (optional): Anzahl der anzuzeigenden Seiten. Standard ist `20`, maximal `100`
- **hasDarkBackground** (optional): Erzwingt das Dark-Mode-Styling. Wird automatisch vom Seitenhintergrund erkannt, wenn nicht gesetzt

## Erweiterte Beispiele

### Benutzerdefinierte Anzahl

[inline-code-attrs-start title = 'Recent Discussions mit benutzerdefinierter Anzahl'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

### Dunkelmodus erzwingen

[inline-code-attrs-start title = 'Recent Discussions mit Dunkelmodus'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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
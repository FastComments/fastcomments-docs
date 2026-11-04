Unten ist der Vanilla JS-Code zur Installation des Summary Widgets. Die React-Bibliothek enthält dieses Widget ebenfalls.

[inline-code-attrs-start title = 'Summary Widget Installation'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-reviews-summary.min.js"></script>
<div id="summary-widget"></div>
<script>
    window.FastCommentsReviewsSummaryWidget(document.getElementById('summary-widget'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

Das Widget findet automatisch die Fragen, die in der Zusammenfassung angezeigt werden sollen, basierend auf der entsprechenden Widget-Konfiguration für diese Seite/Website.

### Kommentare nach Bewertung filtern

Wenn das Kommentar-Widget für dieselbe Seite ebenfalls auf der Seite vorhanden ist und so konfiguriert ist, dass die Antwort jedes Kommentators in seinem Kommentar angezeigt wird, werden die Zeilen der Aufschlüsselung klickbar. Durch Klicken auf eine Zeile, z. B. „5", wird das Kommentar-Widget auf die Kommentare gefiltert, die diese Antwort gegeben haben. Ein erneutes Klicken auf die Zeile oder auf das X am „Showing only"-Chip im Kommentar-Widget zeigt wieder alle Kommentare an. Dies erfordert keine Konfiguration: Das Summary Widget findet das Kommentar-Widget mit derselben `tenantId` und `urlId`.

Dies funktioniert für Sterne-, Daumen-, Emoji-, Skalen- und Einfachauswahl-Fragen. Die Aufschlüsselung, die das Kommentar-Widget über dem Kommentarfeld anzeigt, kann auf dieselbe Weise angeklickt werden.

Wenn Sie das Widget in einer unserer anderen Bibliotheken benötigen, in der es nicht vorhanden ist, erstellen Sie ein Support-Ticket, damit wir wissen, dass wir es hinzufügen sollen.
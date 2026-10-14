Unten finden Sie den Vanilla‑JS‑Code zur Installation des Summary‑Widgets. Die React‑Bibliothek enthält dieses Widget ebenfalls.

[inline-code-attrs-start title = 'Installation des Summary-Widgets'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-reviews-summary.min.js"></script>
<div id="summary-widget"></div>
<script>
    window.FastCommentsReviewsSummaryWidget(document.getElementById('summary-widget'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

Das Widget findet automatisch die Fragen, die in der Zusammenfassung angezeigt werden sollen, basierend auf der entsprechenden Widget‑Konfiguration für diese Seite/Website.

### Filtering Comments by Rating

Wenn das Kommentar‑Widget für dieselbe Seite ebenfalls auf der Seite ist und so konfiguriert ist, dass die Antwort jedes Kommentators in seinem Kommentar angezeigt wird, werden die Zeilen der Aufschlüsselung anklickbar. Das Anklicken einer Zeile, zum Beispiel „5“, filtert das Kommentar‑Widget auf die Kommentare, die diese Antwort gegeben haben. Das erneute Anklicken der Zeile oder das X auf dem „Showing only“-Chip im Kommentar‑Widget zeigt wieder alle Kommentare an. Dies erfordert keine Konfiguration: Das Summary‑Widget findet das Kommentar‑Widget mit derselben `tenantId` und `urlId`.

Dies funktioniert für Stern‑, Daumen‑, Emoji‑, Skalen‑ und Einzelwahl‑Fragen. Die Aufschlüsselung, die das Kommentar‑Widget über dem Kommentarfeld anzeigt, kann auf dieselbe Weise angeklickt werden.

Falls Sie das Widget in einer unserer anderen Bibliotheken benötigen, die es nicht enthält, erstellen Sie ein Support‑Ticket, damit wir wissen, dass es hinzugefügt werden soll.
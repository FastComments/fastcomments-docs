Fügen wir nun unseren Widget-Code hinzu.

Kopieren Sie den folgenden Code. Stellen Sie sicher, dass Sie bei [fastcomments.com](https://fastcomments.com) angemeldet sind und laden Sie diese Seite neu, falls nicht, damit der Code mit Ihren Kontoangaben vorausgefüllt wird; andernfalls wird der Demo-Code angezeigt.

Kopieren wir nun den Code:

[inline-code-attrs-start title = 'Hostinger-Kommentare-Code'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: "demo",
        pageTitle: window.parent.document.title,
        urlId: window.parent.location.href,
        url: window.parent.location.href
    });
</script>
[inline-code-end]

Gehen wir nun zurück zu unserem Seitenersteller und klicken auf `Enter code`:

<div class="screenshot white-bg">
    <div class="title">Code einfügen</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-2.png" alt="Code einfügen" />
</div>

### Hinweis!

Es ist wichtig, dass Sie den obigen Code verwenden und nicht die Codeausschnitte aus anderer Dokumentation, da dieser Ausschnitt speziell für Hostinger angepasst wurde.

Sie sollten jetzt etwas Ähnliches wie das Folgende sehen, das leer erscheint. Das ist erwartetes Verhalten. Bewegen Sie die Maus über den Bereich, in dem das Widget erscheinen sollte:

<div class="screenshot white-bg">
    <div class="title">Code-Widget hinzugefügt</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-3.png" alt="Code-Widget hinzugefügt" />
</div>

Ziehen Sie nun das Widget auf die gewünschte Größe; Sie werden sehen, wie es erscheint:

<div class="screenshot white-bg">
    <div class="title">Größe anpassen</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-4.png" alt="Größe anpassen" />
</div>

...und jetzt Vorschau anzeigen und speichern!

---
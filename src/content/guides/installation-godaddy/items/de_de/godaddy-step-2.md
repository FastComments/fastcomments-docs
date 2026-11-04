Jetzt, da du einen benutzerdefinierten HTML-Block hinzugefügt hast, können wir den FastComments-Widget-Code hinzufügen.

**Verwende den folgenden Code für Godaddy, nicht Code aus anderen Tutorials. Dieser Code ist Godaddy-spezifisch.**

Kopiere den folgenden Code:

[inline-code-attrs-start title = 'Godaddy Kommentare Code-Snippet'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        if (top.location.pathname && top.location.pathname.includes('/f')) {
            FastCommentsUI(document.getElementById('fastcomments-widget'), {
                tenantId: "demo",
                pageTitle: top.document.title,
                url: top.location.href,
                urlId: top.location.pathname
            });
        }
    })();
</script>
[inline-code-end]

Dieses spezielle Code-Snippet wurde für die Kompatibilität mit Godaddy entwickelt und wird außerdem nur auf deinen Blogbeiträgen angezeigt – nicht auf der Startseite.

Füge nun den Code in den Bereich `Custom Code` ein, der in `Schritt Eins` erwähnt wurde.

<div class="screenshot white-bg">
    <div class="title">Code kopieren und einfügen</div>
    <img class="screenshot-image" src="/images/installation-guides/godaddy-step-2-code-added.png" alt="Code kopieren und einfügen" />
</div>

Klicke oben rechts auf Fertig:

<div class="screenshot white-bg">
    <div class="title">Auf Fertig klicken</div>
    <img class="screenshot-image" src="/images/installation-guides/godaddy-step-2-done.png" alt="Auf Fertig klicken" />
</div>

Das war Schritt zwei!
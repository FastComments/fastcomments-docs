Now that you've added a custom HTML block, we can add the FastComments widget code.

**Verwenden Sie den folgenden Code für Godaddy, nicht den Code aus anderen Anleitungen. Dieser Code ist Godaddy-spezifisch.**

Copy the following code:

[inline-code-attrs-start title = 'Godaddy Kommentar-Codeausschnitt'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

This specific code snippet is designed to be compatible with Godaddy, and will also only show on your blog posts - not the homepage.

Now paste the code into the `Custom Code` area mentioned in `Step One`.

<div class="screenshot white-bg">
    <div class="title">Code kopieren und einfügen</div>
    <img class="screenshot-image" src="/images/installation-guides/godaddy-step-2-code-added.png" alt="Code kopieren und einfügen" />
</div>

Klicken Sie oben rechts auf Fertig:

<div class="screenshot white-bg">
    <div class="title">Klicken Sie auf Fertig</div>
    <img class="screenshot-image" src="/images/installation-guides/godaddy-step-2-done.png" alt="Klicken Sie auf Fertig" />
</div>

Das war's für Schritt zwei!
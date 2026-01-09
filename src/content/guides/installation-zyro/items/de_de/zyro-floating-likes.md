FastComments unterstützt auch das Page Reacts (aka Floating Like button) Widget für Zyro.

Sie können es in Aktion in der unteren rechten Ecke dieser Seite sehen!

1. Zuerst den Code kopieren:

[inline-code-attrs-start title = 'Zyro Floating-Likes-Code'; type = 'bash'; useDemoTenant = false; isFunctional = false; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-page-likes-floating.min.js?v=2" async></script>
<div id="fastcomments-page-likes-floating"></div>
<script>
    (function () {
        function tryLoad() {
            if (typeof window.FastCommentsEmbedPageLikesFloating === 'function') {
                window.FastCommentsEmbedPageLikesFloating(document.getElementById('fastcomments-page-likes-floating'), {
                    tenantId: "demo"
                });
            } else {
                setTimeout(tryLoad, 50);
            }
        }

        tryLoad();
    })();
</script>
[inline-code-end]

2. Öffnen Sie dann in Zyro den Site-Builder.
3. Gehen Sie unten links zu Website-Einstellungen.
4. Wählen Sie Integrationen.
5. Fügen Sie den neuen Code ans *Ende* des `Custom code`-Feldes ein und veröffentlichen Sie Ihre Website.
6. Sie werden das Widget nicht im Vorschaumodus sehen, aber es erscheint in der veröffentlichten Version der Website.
FastComments unterstützt außerdem das Page Reacts (auch bekannt als schwebender Like-Button) Widget für Hostinger.

Sie können es unten rechts auf dieser Seite in Aktion sehen!

### Hinweis!

Diese Anweisungen gelten für den Hostinger Site Builder. Wenn Sie Hostinger *WordPress* verwenden, dann kopieren Sie einfach den folgenden Code und fügen ihn Ihrer WordPress-Seite
mit [WPCode](https://wordpress.org/plugins/insert-headers-and-footers/) hinzu, einem kostenlosen und einfachen Plugin zum Einfügen kleiner Code-Snippets auf Ihrer Seite.

1. Zuerst den Code kopieren:

[inline-code-attrs-start title = 'Hostinger Schwebender Like-Button Code'; type = 'bash'; useDemoTenant = false; isFunctional = false; type = 'html';  inline-code-attrs-end]
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

2. Öffnen Sie dann im Hostinger den Site Builder.
3. Gehen Sie unten links zu den Website-Einstellungen.
4. Wählen Sie Integrationen.
5. Fügen Sie den neuen Code an das *Ende* des Feldes `Custom code` ein und veröffentlichen Sie Ihre Seite.
6. Sie werden das Widget nicht im Vorschaumodus sehen, aber es erscheint in der veröffentlichten Version der Seite.
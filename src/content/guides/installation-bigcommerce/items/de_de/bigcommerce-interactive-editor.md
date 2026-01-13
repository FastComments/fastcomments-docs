Es wird nicht empfohlen, FastComments über den Page Builder von BigCommerce hinzuzufügen, da der Code dann manuell zu jeder gewünschten Seite hinzugefügt werden muss.

Wenn dies jedoch gewünscht ist, muss der folgende Codeausschnitt verwendet werden. Codebeispiele aus anderen Tutorials funktionieren aufgrund der Funktionsweise von BigCommerce nicht:

[inline-code-attrs-start title = 'BigCommerce Page Builder-Snippet'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        let loaded = false;
        function attemptLoad() {
            if (loaded) {
                return;
            }
            if (!window.FastCommentsUI) {
                return;
            }
            FastCommentsUI(document.getElementById('fastcomments-widget'), {
                tenantId: "demo"
            });
            loaded = true;
        }
        attemptLoad();
        const interval = setInterval(function () {
            attemptLoad();
            if (loaded) {
                clearInterval(interval);
            }
        }, 300);
    })();
</script>
[inline-code-end]

---
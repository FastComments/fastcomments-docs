---
Es wird nicht empfohlen, FastComments über den Page Builder von BigCommerce hinzuzufügen, da der Code dann manuell zu jeder gewünschten Seite hinzugefügt werden muss.

Wenn dies jedoch gewünscht ist, muss der folgende Code-Schnipsel verwendet werden. Code-Schnipsel aus anderen Tutorials funktionieren aufgrund der Eigenheiten von BigCommerce nicht:

[inline-code-attrs-start title = 'Snippet für den BigCommerce Page Builder'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo"
    }];
</script>
[inline-code-end]

---
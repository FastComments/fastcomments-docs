Als Nächstes fügen wir den FastComments-Widget-Code zu Ihrer Website hinzu. Dieser Code sucht nach allen Formularen mit dem Titel `FastComments Goes Here` und
ersetzt sie durch FastComments.

So gehen wir zu `Settings` unten links im Site-Editor:

<div class="screenshot white-bg">
    <div class="title">Einstellungen öffnen</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-open-settings.png" alt="Einstellungen öffnen" />
</div>

Öffnen Sie den Abschnitt `Custom Head Code`:

<div class="screenshot white-bg">
    <div class="title">Custom Head Code öffnen</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-open-custom-head-code.png" alt="Custom Head Code öffnen" />
</div>

Für Ionos benötigen wir eine **spezielle Version** des FastComments-Widget-Codes. Codeausschnitte aus **anderen Tutorials funktionieren nicht.**

Kopieren Sie nun den folgenden Code:

[inline-code-attrs-start title = 'Ionos FastComments-Snippet'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<script>
    (function () {
        let loaded = false;
        let interval = null;

        function attemptLoad() {
            const nodes = document.querySelectorAll('h2');

            nodes.forEach(function (node) {
                if (node.innerText && node.innerText.trim() === 'FastComments Goes Here') {
                    // wir erhalten das Element, das nicht die volle Breite hat
                    const target = node.parentNode.parentNode.parentNode.parentNode.parentNode;
                    target.innerHTML = '';
                    FastCommentsUI(target, {
                        tenantId: "demo"
                    });
                    interval && clearInterval(interval);
                    loaded = true;
                }
            });
        }

        attemptLoad();
        if (!loaded) {
            interval = setInterval(attemptLoad, 300);
        }
    })();
</script>
[inline-code-end]

...und fügen Sie es wie gezeigt ein:

<div class="screenshot white-bg">
    <div class="title">Einfügen und speichern</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-paste-and-save.png" alt="Einfügen und speichern" />
</div>

---
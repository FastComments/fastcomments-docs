Vervolgens gaan we de FastComments-widgetcode aan je site toevoegen. Deze code zoekt naar alle formulieren met de titel `FastComments Goes Here` en
vervangt deze door FastComments.

Ga naar `Settings` linksonder in de site-editor:

<div class="screenshot white-bg">
    <div class="title">Open instellingen</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-open-settings.png" alt="Open instellingen" />
</div>

Open de sectie `Custom Head Code`:

<div class="screenshot white-bg">
    <div class="title">Open 'Custom Head Code'</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-open-custom-head-code.png" alt="Open 'Custom Head Code'" />
</div>

Voor Ionos hebben we een **speciale versie** van de FastComments-widgetcode nodig. Codesnippets uit **andere handleidingen zullen niet werken.**

Kopieer de volgende code:

[inline-code-attrs-start title = 'Ionos FastComments-fragment'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
                    // we krijgen het element dat niet de volledige breedte heeft
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

...en plak het zoals getoond:

<div class="screenshot white-bg">
    <div class="title">Plak en opslaan</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-paste-and-save.png" alt="Plak en opslaan" />
</div>

---
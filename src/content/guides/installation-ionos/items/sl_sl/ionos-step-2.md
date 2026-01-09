Nato bomo na vaše spletno mesto dodali kodo pripomočka FastComments. Ta koda bo poiskala vse obrazce z naslovom `FastComments Goes Here` in jih nadomestila s FastComments.

Torej pojdimo v `Settings` v spodnjem levem kotu urejevalnika spletnega mesta:

<div class="screenshot white-bg">
    <div class="title">Odprite nastavitve</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-open-settings.png" alt="Odprite nastavitve" />
</div>

Odprite razdelek `Custom Head Code`:

<div class="screenshot white-bg">
    <div class="title">Odprite Prilagojeno kodo v glavi</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-open-custom-head-code.png" alt="Odprite Prilagojeno kodo v glavi" />
</div>

Za Ionos potrebujemo **posebno različico** kode pripomočka FastComments. Izrezki kode iz **drugih vadnic ne bodo delovali.**

Zdaj kopirajte naslednjo kodo:

[inline-code-attrs-start title = 'Ionos FastComments odrezek'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
                    // dobimo element, ki ni polne širine
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

...in ga prilepite, kot je prikazano:

<div class="screenshot white-bg">
    <div class="title">Prilepi in shrani</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-paste-and-save.png" alt="Prilepi in shrani" />
</div>

---
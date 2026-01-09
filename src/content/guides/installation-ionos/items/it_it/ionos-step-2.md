Successivamente aggiungeremo il codice del widget FastComments al tuo sito. Questo codice cercherà tutti i moduli con il titolo `FastComments Goes Here` e
li sostituirà con FastComments.

Quindi andiamo su `Settings` in basso a sinistra dell'editor del sito:

<div class="screenshot white-bg">
    <div class="title">Apri Impostazioni</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-open-settings.png" alt="Apri Impostazioni" />
</div>

Apri la sezione `Custom Head Code`:

<div class="screenshot white-bg">
    <div class="title">Apri Custom Head Code</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-open-custom-head-code.png" alt="Apri Custom Head Code" />
</div>

Per Ionos abbiamo bisogno di una versione **speciale** del codice del widget FastComments. Gli snippet di codice provenienti da **altri tutorial non funzioneranno.**

Ora copia il codice seguente:

[inline-code-attrs-start title = 'Snippet FastComments per Ionos'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
                    // otteniamo l'elemento che non occupa tutta la larghezza
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

...e incollalo come mostrato:

<div class="screenshot white-bg">
    <div class="title">Incolla e Salva</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-paste-and-save.png" alt="Incolla e Salva" />
</div>

---
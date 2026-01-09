Questo esempio utilizza codice personalizzato per essere compatibile con Wix. **Non potrai usare gli snippet di codice FastComments di altri tutorial.**

Apri il modulo per aggiungere la nostra finestra di dialogo HTML personalizzata cliccando `Enter Code` e selezionando `HTML`:

<div class="screenshot white-bg">
    <div class="title">Passo 3: Apri la finestra di dialogo HTML</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-open-form.png" alt="Passo 3: Apri la finestra di dialogo HTML" />
</div>

Copia lo snippet HTML seguente e incollalo nella finestra di dialogo, poi clicca `Update`:

[inline-code-attrs-start title = 'Snippet di codice Wix per i commenti'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const config = {
            tenantId: "demo"
        };
        const instance = FastCommentsUI(document.getElementById('fastcomments-widget'), config);
        window.onmessage = (event) => {
            if (event.data) {
                if (event.data.action === 'reload') {
                    console.log('Updating FastComments:', event.data.url);
                    config.urlId = event.data.url;
                    config.url = event.data.url;
                    instance.update(config);
                }
            }
        }
    })();
</script>
[inline-code-end]

<div class="screenshot white-bg">
    <div class="title">Passo 3: Incolla e Salva</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-paste-and-save.png" alt="Passo 3: Incolla e Salva" />
</div>

Ora dovresti vedere una versione molto piccola del widget dei commenti:

<div class="screenshot white-bg">
    <div class="title">Passo 3: Il risultato</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-result.png" alt="Passo 3: Il risultato" />
</div>

Successivamente posizioneremo e dimensioneremo questo per adattarlo alla nostra pagina.
Nella sezione **Footer** della scheda Codice personalizzato, incolla il codice seguente:

[inline-code-attrs-start title = 'Snippet di codice per commenti live di Typeflo.io'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js" async></script>
<script>
    (function () {
        function load() {
            let target = null;
            let lastInstance;
            if (document.querySelector('.fastcomments-widget')) {
                setTimeout(load, 1000);
                return;
            }
            if (lastInstance) {
                lastInstance.destroy();
            }
            if (window.FastCommentsUI) {
                const newElement = document.createElement('div');
                newElement.classList.add('fastcomments-widget');
                const subscribeSection = document.querySelector('.nc-SectionSubscribe2');
                if (subscribeSection) {
                    subscribeSection.parentNode.insertBefore(newElement, subscribeSection);
                    target = newElement;
                } else {
                    const fullWidthSection = document.querySelector('.container.w-full');
                    if (fullWidthSection) {
                        fullWidthSection.prepend(newElement);
                        target = newElement;
                    }
                }
            }
            if (target) {
                lastInstance = FastCommentsUI(target, {
                    "tenantId": "demo"
                });
            }
            setTimeout(load, 1000);
        }

        load();
    })();
</script>
[inline-code-end]

<div class="screenshot white-bg">
    <div class="title">Incolla il codice nella sezione Footer</div>
    <img class="screenshot-image" src="/images/installation-guides/typeflo-step-3-paste-code-in-footer-section.png" alt="Incolla il codice FastComments nella sezione Footer" />
</div>

Dopo aver incollato il codice, fai clic sul pulsante **Salva** per applicare le modifiche.

Nota: questo codice include una logica per posizionare dinamicamente il widget FastComments nel punto ottimale dei tuoi post sul blog Typeflo.io. Altri snippet di codice non funzioneranno correttamente con il layout di Typeflo.io.

Ricordati di sostituire `'demo'` con il tuo reale ID tenant FastComments dopo la registrazione. Se hai effettuato l'accesso a FastComments.com, dovrebbe essere già sostituito.
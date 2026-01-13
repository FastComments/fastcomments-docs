---
Ora possiamo copiare lo snippet di codice seguente (usa il pulsante di copia in alto a destra dello snippet):

[inline-code-attrs-start title = 'Codice commenti del blog Squarespace'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // l'id del tuo account

        function tryLoad() {
            // prova a caricare per diversi layout
            let targetDiv = document.querySelector('.blog-item-comments-content');
            if (!targetDiv) {
                targetDiv = document.getElementById('fastcomments-widget');
            }
            window.FastCommentsUI(targetDiv, {
                tenantId
            });
        }

        tryLoad();
    })();
</script>

[inline-code-end]

...poi incollalo nell'area del codice e clicca salva:

<div class="screenshot white-bg">
    <div class="title">Incolla e salva</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-step-2-1-add-code-and-save.png" alt="Incolla e salva" />
</div>

---
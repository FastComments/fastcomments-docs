Ora che hai aggiunto un blocco HTML personalizzato, possiamo aggiungere il codice del widget FastComments.

**Usa il seguente codice per Godaddy, non il codice di altri tutorial. Questo codice è specifico per Godaddy.**

Copia il seguente codice:

[inline-code-attrs-start title = 'Snippet di codice per i commenti Godaddy'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        if (top.location.pathname && top.location.pathname.includes('/f')) {
            FastCommentsUI(document.getElementById('fastcomments-widget'), {
                tenantId: "demo",
                pageTitle: top.document.title,
                url: top.location.href,
                urlId: top.location.pathname
            });
        }
    })();
</script>
[inline-code-end]

Questo specifico frammento di codice è progettato per essere compatibile con Godaddy e verrà mostrato solo nei tuoi post del blog, non nella homepage.

Ora incolla il codice nell'area `Custom Code` menzionata in `Step One`.

<div class="screenshot white-bg">
    <div class="title">Copia e incolla il codice</div>
    <img class="screenshot-image" src="/images/installation-guides/godaddy-step-2-code-added.png" alt="Copia e incolla il codice" />
</div>

Premi Fatto in alto a destra:

<div class="screenshot white-bg">
    <div class="title">Clicca su Fatto</div>
    <img class="screenshot-image" src="/images/installation-guides/godaddy-step-2-done.png" alt="Clicca su Fatto" />
</div>

Questo è tutto per il secondo passo!

---
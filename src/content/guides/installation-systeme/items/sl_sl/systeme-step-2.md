Zdaj bomo kopirali našo kodo. Če v odseku kode na vrstici 6 piše `tenantId: "demo"` se prijavite v svoj račun FastComments
in nato osvežite to stran, da bo kopiran odsek kode vseboval ID vašega računa.

[inline-code-attrs-start title = 'Systeme.io Vstavek'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Zdaj prilepite to v urejevalnik in kliknite Shrani:

<div class="screenshot white-bg">
    <div class="title">Dodajte kodo FastComments</div>
    <img class="screenshot-image" src="/images/installation-guides/systeme-add-code.png" alt="Dodajte kodo FastComments" />
</div>

... nato shranite svoje spletno mesto. To je vse!
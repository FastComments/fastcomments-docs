---
We gaan nu onze code kopiëren. Als het codefragment `tenantId: "demo"` op regel 6 aangeeft, moet je inloggen op je FastComments-account en vervolgens deze pagina verversen zodat het gekopieerde codefragment je account-id bevat.

[inline-code-attrs-start title = 'Systeme.io-fragment'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: "demo"
    });
</script>
[inline-code-end]

Plak het nu in de editor en klik op opslaan:

<div class="screenshot white-bg">
    <div class="title">Voeg de FastComments-code toe</div>
    <img class="screenshot-image" src="/images/installation-guides/systeme-add-code.png" alt="Voeg de FastComments-code toe" />
</div>

... sla vervolgens je site op. Dat is alles!

---
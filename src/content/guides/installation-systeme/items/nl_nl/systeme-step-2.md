Now we're going to copy our code. If the code snippet says `tenantId: "demo"` on line 6 then you should log into your FastComments account
en vervolgens deze pagina vernieuwen zodat het gekopieerde codefragment je account-id bevat.

[inline-code-attrs-start title = 'Fragment voor Systeme.io'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Plak het nu in de editor en klik op opslaan:

<div class="screenshot white-bg">
    <div class="title">Voeg de FastComments-code toe</div>
    <img class="screenshot-image" src="/images/installation-guides/systeme-add-code.png" alt="Voeg de FastComments-code toe" />
</div>

... sla vervolgens je site op. Dat is alles!

---
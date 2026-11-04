Teraz skopiujemy nasz kod. Jeśli fragment kodu zawiera `tenantId: "demo"` w linii 6, powinieneś zalogować się na swoje konto FastComments
a następnie odświeżyć tę stronę, aby skopiowany fragment kodu zawierał identyfikator Twojego konta.

[inline-code-attrs-start title = 'Fragment kodu Systeme.io'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Teraz wklej go do edytora i kliknij zapisz:

<div class="screenshot white-bg">
    <div class="title">Dodaj kod FastComments</div>
    <img class="screenshot-image" src="/images/installation-guides/systeme-add-code.png" alt="Dodaj kod FastComments" />
</div>

... a następnie zapisz swoją stronę. To wszystko!
Сада ћемо копирати наш код. Ако исечак кода на линији 6 пише `tenantId: "demo"` онда треба да се пријавите на свој FastComments налог
и затим освежите ову страницу да би копирани исечак кода имао ваш идентификатор налога.

[inline-code-attrs-start title = 'Исечак Systeme.io'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: "demo"
    });
</script>
[inline-code-end]

Сада налепите у уређивач и кликните сачувај:

<div class="screenshot white-bg">
    <div class="title">Додајте FastComments код</div>
    <img class="screenshot-image" src="/images/installation-guides/systeme-add-code.png" alt="Додајте FastComments код" />
</div>

... затим сачувајте ваш сајт. То је то!
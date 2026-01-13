Сада када сте додали прилагођени HTML блок, можемо додати FastComments видгет код.

**Користите следећи код за Godaddy, а не код из других туторијала. Овај код је специфичан за Godaddy.**

Копирајте следећи код:

[inline-code-attrs-start title = 'Исечак кода за Godaddy коментаре'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Овај конкретни исечак кода је дизајниран да буде компатибилан са Godaddy-јем, и приказиваће се само на вашим блог постовима — не на почетној страници.

Сада налепите код у област `Custom Code` поменуту у `Step One`.

<div class="screenshot white-bg">
    <div class="title">Копирајте и налепите код</div>
    <img class="screenshot-image" src="/images/installation-guides/godaddy-step-2-code-added.png" alt="Копирајте и налепите код" />
</div>

Кликните на Done у горњем десном углу:

<div class="screenshot white-bg">
    <div class="title">Кликните на Done</div>
    <img class="screenshot-image" src="/images/installation-guides/godaddy-step-2-done.png" alt="Кликните на Done" />
</div>

То је то за други корак!

---
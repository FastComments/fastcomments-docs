Сада ћемо додати код нашег видгета.

Копирајте код испод. Уверите се да сте пријављени на [fastcomments.com](https://fastcomments.com) 
и освежите ову страницу ако нисте, како би се код попунио подацима вашег налога, у супротном ће бити приказан демо код.

Сада копирајмо код:

[inline-code-attrs-start title = 'Код за коментаре Hostinger'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: "demo",
        pageTitle: window.parent.document.title,
        urlId: window.parent.location.href,
        url: window.parent.location.href
    });
</script>
[inline-code-end]

Сада се вратимо у наш алат за прављење сајта и кликнемо на `Enter code`:

<div class="screenshot white-bg">
    <div class="title">Унесите код</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-2.png" alt="Унесите код" />
</div>

### Напомена!

Важно је да користите горе наведени код, а не исечке кода из друге документације, јер је овај исечак посебно направљен за Hostinger.

Сада би требало да имате нешто попут следећег, што изгледа празно. То је очекивано. Померите миша преко подручја где би видгет требао бити:

<div class="screenshot white-bg">
    <div class="title">Видгет кода додат</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-3.png" alt="Видгет кода додат" />
</div>

Сада превуците видгет да постигнете жељену величину, видећете да се појављује:

<div class="screenshot white-bg">
    <div class="title">Промените величину</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-4.png" alt="Промените величину" />
</div>

...и сада прегледајте и сачувајте!
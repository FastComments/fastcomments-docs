Хајде да сада додамо код нашег видгета.

Копирајте код испод. Уверите се да сте пријављени на [fastcomments.com](https://fastcomments.com) 
и поново учитајте ову страницу ако нисте, тако да ће код бити аутоматски попуњен вашим подацима са налога; у супротном ће се приказати демо код.

Сада копирајмо код:

[inline-code-attrs-start title = 'Zyro код коментара'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Сада хајде да се вратимо у наш алат за прављење сајта и кликнемо на `Enter code`:

<div class="screenshot white-bg">
    <div class="title">Унеси код</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-2.png" alt="Унеси код" />
</div>

### Напомена!

Важно је да користите горњи код, а не исечке из друге документације, јер је овај исечак посебно прилагођен за Zyro.

Сада бисте требали имати нешто слично следећем, што изгледа празно. То је очекивано. Померите курсор миша преко области где би видгет требало да буде:

<div class="screenshot white-bg">
    <div class="title">Код видгета додат</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-3.png" alt="Код видгета додат" />
</div>

Сада превуците видгет да постигнете жељену величину, видећете га како се појављује:

<div class="screenshot white-bg">
    <div class="title">Промени величину</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-4.png" alt="Промени величину" />
</div>

...и сада прегледајте и сачувајте!

---
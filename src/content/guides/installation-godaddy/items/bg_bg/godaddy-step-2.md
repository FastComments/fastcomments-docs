След като добавихте персонализиран HTML блок, можем да добавим кода на FastComments уиджета.

**Използвайте следния код за Godaddy, не кода от други уроци. Този код е специфичен за Godaddy.**

Копирайте следния код:

[inline-code-attrs-start title = 'Кодов фрагмент за коментари на Godaddy'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Този конкретен кодов фрагмент е предназначен да бъде съвместим с Godaddy и ще се показва само в публикациите на вашия блог - не на началната страница.

Сега поставете кода в областта `Custom Code`, спомената в `Step One`.

<div class="screenshot white-bg">
    <div class="title">Копирайте и поставете кода</div>
    <img class="screenshot-image" src="/images/installation-guides/godaddy-step-2-code-added.png" alt="Копирайте и поставете кода" />
</div>

Натиснете Done в горния десен ъгъл:

<div class="screenshot white-bg">
    <div class="title">Натиснете 'Done'</div>
    <img class="screenshot-image" src="/images/installation-guides/godaddy-step-2-done.png" alt="Натиснете 'Done'" />
</div>

Това е всичко за втората стъпка!
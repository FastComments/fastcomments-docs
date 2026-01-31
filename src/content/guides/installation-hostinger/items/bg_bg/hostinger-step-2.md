Нека сега да добавим кода на нашия уиджет.

Копирайте кода по-долу. Ще искате да се уверите, че сте влезли в [fastcomments.com](https://fastcomments.com) 
и да презаредите тази страница, ако не сте, така че кодът да бъде предварително попълнен с информацията от вашия акаунт, в противен случай ще се покаже демонстрационен код.

Сега нека копираме кода:

[inline-code-attrs-start title = 'Код за коментари на Hostinger'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo",
        pageTitle: window.parent.document.title,
        urlId: window.parent.location.href,
        url: window.parent.location.href
    }];
</script>
[inline-code-end]

Сега нека да се върнем в конструктора на сайта и да кликнем `Enter code`:

<div class="screenshot white-bg">
    <div class="title">Въведете код</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-2.png" alt="Въведете код" />
</div>

### Забележка!

Важно е да използвате горния код, а не фрагментите от друга документация, тъй като този фрагмент е разработен специално за Hostinger.

Сега би трябвало да имате нещо подобно на следното, което изглежда празно. Това е очаквано. Преместете мишката върху областта, където трябва да се появи уиджетът:

<div class="screenshot white-bg">
    <div class="title">Добавен уиджет с код</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-3.png" alt="Добавен уиджет с код" />
</div>

Сега плъзнете уиджета до желания размер; ще го видите да се появява:

<div class="screenshot white-bg">
    <div class="title">Преоразмерете го</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-4.png" alt="Преоразмерете го" />
</div>

...и сега прегледайте и запазете!
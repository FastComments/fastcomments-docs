Сега нека добавим кода на нашия уиджет.

Копирайте кода по-долу. Уверете се, че сте влезли в [fastcomments.com](https://fastcomments.com) и презаредете тази страница, ако не сте, за да кодът бъде предварително попълнен с информацията за вашия акаунт, в противен случай ще се покаже демо код.

Сега нека копираме кода:

[inline-code-attrs-start title = 'Код за коментари на Zyro'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Сега се върнете в редактора на сайта и кликнете `Enter code`:

<div class="screenshot white-bg">
    <div class="title">Въведете код</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-2.png" alt="Въведете код" />
</div>

### Забележка!

Важно е да използвате горния код, а не кодовите фрагменти от друга документация, тъй като този фрагмент е създаден специално за Zyro.

Трябва да имате нещо подобно на следното, което изглежда празно. Това е очаквано. Преместете мишката си върху зоната, където трябва да се появи уиджета:

<div class="screenshot white-bg">
    <div class="title">Уиджетът е добавен</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-3.png" alt="Уиджетът е добавен" />
</div>

Сега плъзнете уиджета до желания размер — ще видите как се появява:

<div class="screenshot white-bg">
    <div class="title">Променете размера</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-4.png" alt="Променете размера" />
</div>

...и сега прегледайте и запазете!
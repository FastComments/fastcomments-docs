---
Сега нека добавим кода на нашия уиджет.

Копирайте кода по-долу. Уверете се, че сте влезли в [fastcomments.com](https://fastcomments.com) 
и презаредете тази страница, ако не сте, за да кодът бъде предварително попълнен с информацията на вашия акаунт, в противен случай ще се покаже демонстрационният код.

Сега нека копираме кода:

[inline-code-attrs-start title = 'Код за коментари на Zyro'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Сега нека се върнем в конструктора на сайта и кликнем `Enter code`:

<div class="screenshot white-bg">
    <div class="title">Въведете код</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-2.png" alt="Въведете код" />
</div>

### Забележка!

Важно е да използвате горния код, а не фрагментите от други документи, тъй като този фрагмент е създаден специално
за Zyro.

Сега трябва да имате нещо подобно на следното, което изглежда празно. Това е очаквано. Преместете мишката върху областта
където трябва да бъде уиджетът:

<div class="screenshot white-bg">
    <div class="title">Уиджетът с код е добавен</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-3.png" alt="Уиджетът с код е добавен" />
</div>

Сега плъзнете уиджета до желаната големина, ще видите как се появява:

<div class="screenshot white-bg">
    <div class="title">Променете размера</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-4.png" alt="Променете размера" />
</div>

...и сега прегледайте и запазете!

---
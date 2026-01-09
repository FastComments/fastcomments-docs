Сега можем да копираме следния фрагмент от код. Използвайте бутона за копиране, който се появява в горния десен ъгъл на фрагмента.

Има няколко неща, които можете да конфигурирате в кода, вижте редове 4 до 7.

[inline-code-attrs-start title = 'Squarespace код за една страница'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // вашият идентификатор на акаунта

        function tryLoad() {
            window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
                tenantId
            });
        }

        tryLoad();
    })();
</script>

[inline-code-end]

Трябва да изглежда така:

<div class="screenshot white-bg">
    <div class="title">Поставете и запазете</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-specific-page-step-2-1-add-code-and-save.png" alt="Поставете и запазете" />
</div>

Сега кликнете върху бутона Save в горния десен ъгъл.

Имайте предвид, че опцията `Preview in Safe Mode` няма да работи, но уиджетът ще се появи, когато посетите сайта си.

Ако имате проблеми, уверете се, че към дъното не пише `"tenantId": "demo"`. Трябва да показва вашия tenant id, ако сте влезли в профила си. Ако не, свържете се с екипа за поддръжка.
Цей приклад використовує власний код для сумісності з Wix. **Ви не зможете використовувати фрагменти коду FastComments з інших підручників.**

Open the form to add our custom HTML dialog by clicking `Enter Code` and selecting `HTML`:

<div class="screenshot white-bg">
    <div class="title">Крок 3: Відкрийте HTML-діалог</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-open-form.png" alt="Крок 3: Відкрийте HTML-діалог" />
</div>

Скопіюйте наведений HTML-фрагмент і вставте його в діалог, і натисніть Update:

[inline-code-attrs-start title = 'Фрагмент коду для коментарів Wix'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const config = {
            tenantId: "demo"
        };
        const instance = FastCommentsUI(document.getElementById('fastcomments-widget'), config);
        window.onmessage = (event) => {
            if (event.data) {
                if (event.data.action === 'reload') {
                    console.log('Updating FastComments:', event.data.url);
                    config.urlId = event.data.url;
                    config.url = event.data.url;
                    instance.update(config);
                }
            }
        }
    })();
</script>
[inline-code-end]

<div class="screenshot white-bg">
    <div class="title">Крок 3: Вставте та збережіть</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-paste-and-save.png" alt="Крок 3: Вставте та збережіть" />
</div>

Тепер ви повинні бачити дуже маленьку версію віджету коментарів:

<div class="screenshot white-bg">
    <div class="title">Крок 3: Результат</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-result.png" alt="Крок 3: Результат" />
</div>

Далі ми розташуємо та змінемо розмір цього елемента, щоб він відповідав нашій сторінці.
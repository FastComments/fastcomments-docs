Този пример използва персонализиран код, за да е съвместим с Wix. **Няма да можете да използвате фрагментите с код на FastComments от другите уроци.**

Отворете формуляра за добавяне на нашия персонализиран HTML диалог чрез клик върху `Enter Code` и избор на `HTML`:

<div class="screenshot white-bg">
    <div class="title">Стъпка 3: Отворете HTML диалога</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-open-form.png" alt="Стъпка 3: Отворете HTML диалога" />
</div>

Копирайте следния HTML фрагмент и го поставете в диалога, след което кликнете върху Update:

[inline-code-attrs-start title = 'Кодов фрагмент за коментари в Wix'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
    <div class="title">Стъпка 3: Поставете и запазете</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-paste-and-save.png" alt="Стъпка 3: Поставете и запазете" />
</div>

Сега трябва да видите много миниатюрен вариант на виджета за коментари:

<div class="screenshot white-bg">
    <div class="title">Стъпка 3: Резултатът</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-result.png" alt="Стъпка 3: Резултатът" />
</div>

След това ще го позиционираме и оразмерим, за да пасне на страницата ни.
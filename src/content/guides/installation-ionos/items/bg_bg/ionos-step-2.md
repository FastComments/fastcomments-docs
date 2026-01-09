След това ще добавим кода на FastComments widget към вашия сайт. Този код ще търси всички формуляри със заглавие `FastComments Goes Here` и
ще ги замени с FastComments.

Да отидем в `Settings` в долния ляв ъгъл на редактора на сайта:

<div class="screenshot white-bg">
    <div class="title">Отворете Settings</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-open-settings.png" alt="Отворете Settings" />
</div>

Отворете секцията `Custom Head Code`:

<div class="screenshot white-bg">
    <div class="title">Отворете Custom Head Code</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-open-custom-head-code.png" alt="Отворете Custom Head Code" />
</div>

За Ionos се нуждаем от **специална версия** на кода за FastComments widget. Фрагменти от **други уроци няма да работят.**

Сега копирайте следния код:

[inline-code-attrs-start title = 'Фрагмент FastComments за Ionos'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<script>
    (function () {
        let loaded = false;
        let interval = null;

        function attemptLoad() {
            const nodes = document.querySelectorAll('h2');

            nodes.forEach(function (node) {
                if (node.innerText && node.innerText.trim() === 'FastComments Goes Here') {
                    // взимаме елемента, който не е с пълна ширина
                    const target = node.parentNode.parentNode.parentNode.parentNode.parentNode;
                    target.innerHTML = '';
                    FastCommentsUI(target, {
                        tenantId: "demo"
                    });
                    interval && clearInterval(interval);
                    loaded = true;
                }
            });
        }

        attemptLoad();
        if (!loaded) {
            interval = setInterval(attemptLoad, 300);
        }
    })();
</script>
[inline-code-end]

...и го поставете, както е показано:

<div class="screenshot white-bg">
    <div class="title">Поставете и запазете</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-paste-and-save.png" alt="Поставете и запазете" />
</div>

---
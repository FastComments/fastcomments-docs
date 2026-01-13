---
Далее мы добавим код виджета FastComments на ваш сайт. Этот код будет искать все формы с заголовком `FastComments Goes Here` и
заменять их на FastComments.

Итак, перейдите в `Settings` в нижнем левом углу редактора сайта:

<div class="screenshot white-bg">
    <div class="title">Откройте Settings</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-open-settings.png" alt="Откройте настройки" />
</div>

Откройте раздел `Custom Head Code`:

<div class="screenshot white-bg">
    <div class="title">Откройте Custom Head Code</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-open-custom-head-code.png" alt="Откройте Custom Head Code" />
</div>

Для Ionos нам нужна **специальная версия** кода виджета FastComments. Фрагменты кода из **других руководств не подойдут.**

Теперь скопируйте следующий код:

[inline-code-attrs-start title = 'Сниппет FastComments для Ionos'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
                    // получаем элемент, который не занимает всю ширину
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

...и вставьте его, как показано:

<div class="screenshot white-bg">
    <div class="title">Вставьте и сохраните</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-paste-and-save.png" alt="Вставьте и сохраните" />
</div>

---
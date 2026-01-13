Далі ми додамо код віджета FastComments на ваш сайт. Цей код шукатиме всі форми з заголовком `FastComments Goes Here` і
замінить їх на FastComments.

Тож перейдіть до `Settings` у нижньому лівому куті редактора сайту:

<div class="screenshot white-bg">
    <div class="title">Відкрийте Налаштування</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-open-settings.png" alt="Відкрийте Налаштування" />
</div>

Відкрийте розділ `Custom Head Code`:

<div class="screenshot white-bg">
    <div class="title">Відкрийте розділ Custom Head Code</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-open-custom-head-code.png" alt="Відкрийте розділ Custom Head Code" />
</div>

Для Ionos нам потрібна **спеціальна версія** коду віджета FastComments. Фрагменти коду з **інших підручників не будуть працювати.**

Тепер скопіюйте наступний код:

[inline-code-attrs-start title = 'Фрагмент FastComments для Ionos'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
                    // отримуємо елемент, який не займає повної ширини
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

...і вставте його, як показано:

<div class="screenshot white-bg">
    <div class="title">Вставте та збережіть</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-paste-and-save.png" alt="Вставте та збережіть" />
</div>

---
[related-parameter-start name = 'hasDarkBackground'; type = 'boolean'; related-parameter-end]

Для сайтов, которые позволяют переключать тёмный режим после первоначальной загрузки страницы, это немного сложнее.

Во-первых, во всех текущих версиях библиотеки виджета комментариев (React, Vue) есть примеры переключения тёмного режима в их соответствующих репозиториях.

Для виджета VanillaJS потребуется немного больше работы. Сначала FastCommentsUI возвращает объект с функциями "destroy" и "update".

Мы можем просто вызывать функцию update каждый раз, когда хотим обновить конфигурацию виджета комментариев, как показано ниже. Здесь приведён полный рабочий пример переключения
тёмного режима для виджета VanillaJS.

[inline-code-attrs-start title = 'Полный пример переключения тёмного режима'; inline-code-attrs-end]
[inline-code-start]
<script src="https://fastcomments.com/js/embed-v2.min.js"></script>
<button id="toggle-dark-mode">Toggle Dark Mode</button>
<div id="fastcomments-widget"></div>
<script>
    (function() {
        const button = document.getElementById('toggle-dark-mode');
        const config = {
            tenantId: 'demo',
            hasDarkBackground: false
        };
        const instance = window.FastCommentsUI(document.getElementById('fastcomments-widget'), config);
        button.addEventListener('click', function() {
            config.hasDarkBackground = !config.hasDarkBackground;
            if (config.hasDarkBackground) {
                document.body.classList.add('dark');
            } else {
                document.body.classList.remove('dark');
            }
            instance.update(config);
        });
    })();
</script>
<style>
    body.dark {
        background: #000;
        color: #fff;
    }
</style>
[inline-code-end]

---
[related-parameter-start name = 'hasDarkBackground'; type = 'boolean'; related-parameter-end]

За сайтове, които позволяват превключване на тъмен режим след първоначалното зареждане на страницата, това е малко по-сложно.

Първо, всички текущи версии на библиотеката за коментарен widget (React, Vue) съдържат примери за превключване на тъмен режим в съответните им хранилища.

За VanillaJS уиджет ще трябва да направим още някои неща. Първо, FastCommentsUI връща обект с функциите "destroy" и "update".

Можем просто да извикаме функцията "update" всеки път, когато искаме да обновим конфигурацията на коментарния уиджет, както следва. Ето пълен работещ пример за превключване
на тъмен режим с VanillaJS уиджета.

[inline-code-attrs-start title = 'Пълен пример за превключване на тъмен режим'; inline-code-attrs-end]
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
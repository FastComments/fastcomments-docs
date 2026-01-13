[related-parameter-start name = 'hasDarkBackground'; type = 'boolean'; related-parameter-end]

Для сайтів, які дозволяють перемикати темний режим після початкового завантаження сторінки, це трохи складніше.

По-перше, усі поточні версії бібліотеки віджета коментарів (React, Vue) містять приклади перемикання темного режиму у відповідних репозиторіях.

Для VanillaJS віджета нам потрібно зробити трохи більше роботи. По-перше, FastCommentsUI повертає об'єкт з функціями "destroy" і "update".

Ми можемо просто викликати функцію update щоразу, коли хочемо оновити конфігурацію віджета коментарів, як показано нижче. Ось повний робочий приклад перемикання
темного режиму з VanillaJS віджетом.

[inline-code-attrs-start title = 'Повний приклад перемикання темного режиму'; inline-code-attrs-end]
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
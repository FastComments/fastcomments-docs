[related-parameter-start name = 'hasDarkBackground'; type = 'boolean'; related-parameter-end]

За сајтове који омогућавају пребацивање тамног режима након иницијалног учитавања странице, ово је мало сложеније.

Прво, све тренутне верзије библиотеке Comment widget (React, Vue) имају примере пребацивања тамног режима у својим репозиторијумима.

За VanillaJS widget, мораћемо да урадимо још неколико ствари. Прво, FastCommentsUI враћа објекат са функцијама "destroy" и "update".

Једноставно можемо позвати функцију update сваки пут када желимо да ажурирамо конфигурацију widgetа за коментаре, на следећи начин. Ево комплетног функционалног примера пребацивања
тамног режима са VanillaJS widget-ом.

[inline-code-attrs-start title = 'Комплетан пример пребацивања тамног режима'; inline-code-attrs-end]
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
Фронт-енд фрагменти кода и библиотеке за On-Prem су исти као код SaaS производа. Међутим, морате навести `apiHost` и исправну путању скрипте:

[inline-code-attrs-start title = 'Код коментара за On Prem'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://my.host.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo",
        apiHost: "https://my.host.com"
        // ... такође се може проследити SSO payload итд.
    }];
</script>
[inline-code-end]

Горњи пример је веома једноставан. Такође можемо користити званичне React, Angular, Vue, Svelte и друге библиотеке.

---
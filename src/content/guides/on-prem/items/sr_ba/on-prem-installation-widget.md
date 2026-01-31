Фронтенд фрагменти кода и библиотеке за On-Prem су исти као и за SaaS производ. Међутим, морате назначити `apiHost` и исправну путању до скрипте:

[inline-code-attrs-start title = 'Код коментара за On-Prem'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://my.host.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo",
        apiHost: "https://my.host.com"
        // ... можете такође проследити SSO податке итд.
    }];
</script>
[inline-code-end]

Горе је дат веома једноставан пример. Такође можемо користити званичне (1st-party) библиотеке за React, Angular, Vue, Svelte итд.
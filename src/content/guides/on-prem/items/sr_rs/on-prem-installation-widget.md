Фрагменти фронт-енд кода и библиотеке за On-Prem су исти као и за SaaS производ. Међутим, морате да наведете `apiHost` и тачан пут до скрипте:

[inline-code-attrs-start title = 'Код за коментаре за On Prem'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://my.host.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: "demo",
        apiHost: "https://my.host.com"
        // ... такође можете проследити SSO payload итд.
    });
</script>
[inline-code-end]

Горњи пример је веома једноставан. Такође можемо користити 1st-party библиотеке за React, Angular, Vue, Svelte итд.
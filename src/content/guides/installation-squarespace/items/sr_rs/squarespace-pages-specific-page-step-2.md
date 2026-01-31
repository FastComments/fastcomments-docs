Сада можемо да копирамо следећи исечак кода. Користите дугме за копирање које се појављује у горњем десном углу исечка.

Постоји неколико ствари које можете да конфигуришете у коду, видите линије 4 до 7.

[inline-code-attrs-start title = 'Squarespace код за једну страницу'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: 'demo' // ваш ID налога
    }];
</script>
[inline-code-end]

То би требало да изгледа овако:

<div class="screenshot white-bg">
    <div class="title">Залепите и сачувајте</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-specific-page-step-2-1-add-code-and-save.png" alt="Залепите и сачувајте" />
</div>

Сада кликните на сачувај у горњем десном углу.

Имајте на уму да опција `Preview in Safe Mode` неће радити, али видгет ће се појавити када посетите ваш сајт.

Ако имате проблеме, уверите се да при дну не пише "tenantId": "demo". Требало би да прикаже ваш tenant id ако сте пријављени. Ако није, обратите се подршци.
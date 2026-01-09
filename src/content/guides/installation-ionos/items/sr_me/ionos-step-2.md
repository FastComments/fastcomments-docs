Сљедеће ћемо додати код FastComments видгета на ваш сајт. Овај код ће претражити све формуларе са насловом `FastComments Goes Here` и
замениће их FastComments-ом.

Дакле, идемо на `Settings` у доњем лијевом углу уређивача сајта:

<div class="screenshot white-bg">
    <div class="title">Отворите Подешавања</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-open-settings.png" alt="Отворите Подешавања" />
</div>

Отворите одељак `Custom Head Code`:

<div class="screenshot white-bg">
    <div class="title">Отворите Custom Head Code</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-open-custom-head-code.png" alt="Отворите Custom Head Code" />
</div>

За Ionos нам треба **специјална верзија** FastComments видгет кода. Комадићи кода из **других туторијала неће радити.**

Сада копирајте следећи код:

[inline-code-attrs-start title = 'Ionos FastComments исјечак'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
                    // добијамо елемент који није пуне ширине
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

...и залепите га као што је приказано:

<div class="screenshot white-bg">
    <div class="title">Залепите и сачувајте</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-paste-and-save.png" alt="Залепите и сачувајте" />
</div>

---
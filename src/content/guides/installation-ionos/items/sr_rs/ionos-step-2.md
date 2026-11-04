Затим ћемо додати FastComments виџет код на ваш сајт. Овај код ће претражити све форме са насловом `FastComments Goes Here` и
заменити их са FastComments.

Хајде да отворимо `Settings` у доњем левом углу уредника сајта:

<div class="screenshot white-bg">
    <div class="title">Отварање подешавања</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-open-settings.png" alt="Отварање подешавања" />
</div>

Отворите секцију `Custom Head Code`:

<div class="screenshot white-bg">
    <div class="title">Отварање Custom Head Code</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-open-custom-head-code.png" alt="Отварање Custom Head Code" />
</div>

За Ionos нам је потребна **посебна верзија** FastComments виџет кода. Исечци кода из **других туторијала неће радити.**

Сада копирајте следећи код:

[inline-code-attrs-start title = 'Ionos FastComments исечак'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

...и налепите га као што је приказано:

<div class="screenshot white-bg">
    <div class="title">Налепите и сачувајте</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-paste-and-save.png" alt="Налепите и сачувајте" />
</div>
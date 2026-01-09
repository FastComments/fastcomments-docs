---
Сада можемо копирати следећи исечак кода (користите дугме за копирање у горњем десном углу исечка):

[inline-code-attrs-start title = 'Код коментара за Squarespace блог'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // ваш ID налога

        function tryLoad() {
            // покушај учитавања за различите распореде
            let targetDiv = document.querySelector('.blog-item-comments-content');
            if (!targetDiv) {
                targetDiv = document.getElementById('fastcomments-widget');
            }
            window.FastCommentsUI(targetDiv, {
                tenantId
            });
        }

        tryLoad();
    })();
</script>

[inline-code-end]

...затим залепите у подручје за код и кликните сачувај:

<div class="screenshot white-bg">
    <div class="title">Залепи и сачувај</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-step-2-1-add-code-and-save.png" alt="Залепи и сачувај" />
</div>

---
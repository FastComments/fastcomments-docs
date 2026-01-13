Сега можем да копираме следния фрагмент от код (използвайте бутона за копиране в горния десен ъгъл на фрагмента):

[inline-code-attrs-start title = 'Код за коментари в блога на Squarespace'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // вашият идентификатор на акаунта

        function tryLoad() {
            // опитайте да заредите за различни оформления
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

...след това поставете в областта за код и щракнете върху Запази:

<div class="screenshot white-bg">
    <div class="title">Поставете и запазете</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-step-2-1-add-code-and-save.png" alt="Поставете и запазете" />
</div>

---
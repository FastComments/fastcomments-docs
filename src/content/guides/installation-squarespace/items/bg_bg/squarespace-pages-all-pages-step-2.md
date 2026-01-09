Сега можем да копираме следния фрагмент от код. Използвайте бутона за копиране, който се появява в горния десен ъгъл на фрагмента.

Има няколко неща, които можете да конфигурирате в кода, вижте редове 4 до 7.

[inline-code-attrs-start title = 'Squarespace Код за коментари на всички страници'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // вашият идентификатор на акаунта

        function tryLoad() {
            // опит за зареждане при различни оформления
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

...след това поставете в полето за код и натиснете запазване. Трябва да изглежда така, като кодът е в блока `FOOTER`:

<div class="screenshot white-bg">
    <div class="title">Поставете и запазете</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-all-pages-step-2-1-add-code-and-save.png" alt="Поставете и запазете" />
</div>

Ако имате проблеми, уверете се, че в долната част не пише `"tenantId": "demo"`. Трябва да показва вашия tenant id, ако сте влезли. Ако не, свържете се с поддръжката.
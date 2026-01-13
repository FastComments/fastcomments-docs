[related-parameter-start name = 'urlId'; type = 'string'; related-parameter-end]

Ми розглянули, як `urlId` — це ідентифікатор сторінки або статті, до яких прив'язані коментарі.

Також, нагадаємо, якщо `urlId` не визначено, він за замовчуванням прийматиме поточну URL-адресу сторінки.

А як бути з SPA, або односторінковими застосунками (Single-Page-Applications), де сторінка або вміст, до яких прив'язані коментарі, змінюється динамічно без повного перезавантаження сторінки?

#### Angular, React, Vue тощо

У наших бібліотеках, таких як Angular та React, достатньо оновити властивість `urlId`, передану віджету, щоб віджет коментарів оновився. Ви можете побачити це на прикладі React-додатку, наприклад, <a href="https://github.com/FastComments/fastcomments-react/blob/master/example/src/PaginatedApp.tsx#L39" target="_blank">тут</a>.

#### VanillaJS

Якщо ви використовуєте бібліотеку VanillaJS, це трохи складніше, оскільки немає фреймворку на кшталт Angular або React для обробки прив'язки даних або поширення стану.

Коли ви створюєте екземпляр віджету VanillaJS, він повертає деякі функції, які можна викликати для його оновлення.

Ось робочий приклад, де ми змінюємо хеш сторінки і оновлюємо віджет коментарів:

[inline-code-attrs-start title = 'Приклад зміни хешу сторінки'; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<button id="change-page"></button>
<div id="fastcomments-widget"></div>
<script>
    (function fastCommentsMain() {
        let config = {
            tenantId: 'demo'
        };
        let instance = window.FastCommentsUI(document.getElementById('fastcomments-widget'), config);

        let page = '#page-1';
        function getNextPage() {
            if (page === '#page-1') {
                return '#page-2';
            } else {
                return '#page-1';
            }
        }

        let button = document.getElementById('change-page');
        function nextPage() {
            page = getNextPage();
            button.innerText = 'Go to ' + getNextPage();
            window.location.hash = page;
            let locationString = window.location.toString();

            config.url = locationString; // Ми також оновлюємо url, щоб сповіщення могли посилатися на правильну сторінку
            config.urlId = locationString;

            instance.update(config);
        }
        nextPage();
        button.addEventListener('click', nextPage);
    })();
</script>
[inline-code-end]

---
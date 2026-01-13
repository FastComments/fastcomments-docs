Після того, як ми завантажили zip-файл, розпакуйте його в папку. Я завантажив стандартний `casper.zip` і розпакував його в `Downloads\casper` на Windows.

Далі переконайтеся, що у вас встановлена LTS або новіша версія NodeJS. Ви можете завантажити її тут: https://nodejs.org/en/download/

Після встановлення NodeJS встановіть редактор коду.

Ми рекомендуємо (і самі використовуємо) Webstorm, який можна отримати тут із 30-денною пробною версією (кредитна картка не потрібна): https://www.jetbrains.com/webstorm/

Наступним найкращим безкоштовним варіантом, ймовірно, буде Visual Studio Code: https://code.visualstudio.com/download

Коли ви налаштуєте редактор і відкриєте папку теми в ньому, відкрийте термінал в IDE і виконайте:

[inline-code-attrs-start title = 'Встановити тему'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install
[inline-code-end]

Успішний вивід виглядатиме так (попередження можна ігнорувати):

<div class="screenshot white-bg">
    <div class="title">Успішний вивід npm install</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-2-1-install.png" alt="Успішний вивід npm install" />
</div>

Це налаштує залежності теми для подальших команд, які ми виконаємо. Також експорт залежить від того, що залежності теми встановлені; інакше повторний імпорт не працюватиме належним чином.
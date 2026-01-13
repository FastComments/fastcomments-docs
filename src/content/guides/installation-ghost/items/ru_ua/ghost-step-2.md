Теперь, когда мы скачали zip-файл, распакуйте его в папку. Я скачал стандартный `casper.zip` и распаковал в `Downloads\casper` на Windows.

Далее убедитесь, что у вас установлена LTS или более новая версия NodeJS. Скачать можно здесь: https://nodejs.org/en/download/

После установки NodeJS установите редактор кода.

Мы рекомендуем (и используем) Webstorm, который можно получить здесь с 30‑дневной пробной версией (без кредитной карты): https://www.jetbrains.com/webstorm/

Следующим лучшим бесплатным вариантом, вероятно, будет Visual Studio Code: https://code.visualstudio.com/download

Когда редактор настроен и папка темы открыта в редакторе, откройте терминал в IDE и выполните:

[inline-code-attrs-start title = 'Установить тему'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install
[inline-code-end]

Вывод при успешном выполнении будет выглядеть так (можно игнорировать предупреждения):

<div class="screenshot white-bg">
    <div class="title">Успешный вывод npm install</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-2-1-install.png" alt="Успешный вывод npm install" />
</div>

Это установит зависимости темы для последующих команд, которые мы будем выполнять. Также экспорт зависит от установленных зависимостей темы, в противном случае повторный импорт не будет работать корректно.

---
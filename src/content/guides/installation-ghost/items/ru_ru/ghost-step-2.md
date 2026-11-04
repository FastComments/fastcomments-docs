Теперь, когда мы скачали zip-файл, распакуйте его в папку. Я скачал стандартный `casper.zip` и распаковал его в `Downloads\casper` на Windows.

Далее убедитесь, что у вас установлена LTS или более новая версия NodeJS. Скачать её можно здесь: https://nodejs.org/en/download/

После установки NodeJS вам понадобится редактор кода.

Мы рекомендуем (и сами используем) Webstorm, который можно скачать здесь с 30-дневной пробной версией (кредитная карта не требуется): https://www.jetbrains.com/webstorm/

Следующая лучшая бесплатная альтернатива — Visual Studio Code: https://code.visualstudio.com/download

Когда редактор настроен и папка темы открыта в нём, откройте терминал в IDE и выполните:

[inline-code-attrs-start title = 'Установка темы'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install
[inline-code-end]

Успешный вывод будет выглядеть так (предупреждения можно игнорировать):

<div class="screenshot white-bg">
    <div class="title">Успешный вывод npm install</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-2-1-install.png" alt="Успешный вывод npm install" />
</div>

Это установит зависимости темы для последующих команд, которые мы будем выполнять. Кроме того, экспорт зависит от установленных зависимостей темы, иначе повторный импорт не будет работать корректно.
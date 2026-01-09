Теперь, когда мы скачали zip-файл, распакуйте его в папку. Я скачал стандартный `casper.zip` и распаковал в Downloads\casper на Windows.

Next, you'll want to make sure you have the LTS or newer version of NodeJS installed. You can get that here: https://nodejs.org/en/download/

Once NodeJS is installed, you'll want to install a code editor.

We recommend (and use) Webstorm, which you can get here with a 30-day trial (no credit card needed): https://www.jetbrains.com/webstorm/

The next best free option would probably be Visual Studio Code: https://code.visualstudio.com/download

Once you have your editor setup, and the theme folder open in the editor, open the terminal in the IDE and run:

[inline-code-attrs-start title = 'Установить тему'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install
[inline-code-end]

Успешный вывод будет выглядеть так (предупреждения можно игнорировать):

<div class="screenshot white-bg">
    <div class="title">Успешный вывод npm install</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-2-1-install.png" alt="Успешный вывод npm install" />
</div>

This will set up the theme's dependencies for later commands we will run. Also, the export depends on the theme's dependencies being installed, otherwise the re-import will not work properly.

---
Now that we've downloaded the zip file, extract it to a folder. I've downloaded the default `casper.zip` and extracted to `Downloads\casper` on Windows.

Next, you'll want to make sure you have the LTS or newer version of NodeJS installed. You can get that here: https://nodejs.org/en/download/

Once NodeJS is installed, you'll want to install a code editor.

We recommend (and use) Webstorm, which you can get here with a 30-day trial (no credit card needed): https://www.jetbrains.com/webstorm/

The next best free option would probably be Visual Studio Code: https://code.visualstudio.com/download

Once you have your editor setup, and the theme folder open in the editor, open the terminal in the IDE and run:

[inline-code-attrs-start title = 'התקן את התבנית'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install
[inline-code-end]

Successful output will look like this (you can ignore warnings):

<div class="screenshot white-bg">
    <div class="title">פלט התקנה מוצלח של npm</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-2-1-install.png" alt="פלט התקנה מוצלח של npm" />
</div>

This will set up the theme's dependencies for later commands we will run. Also, the export depends on the theme's dependencies being installed, otherwise the re-import will not work properly.

---
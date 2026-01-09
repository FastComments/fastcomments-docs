Сега, когато сме изтеглили zip файла, разархивирайте го в папка. Аз изтеглих по подразбиране `casper.zip` и го разархивирах в `Downloads\casper` на Windows.

След това ще искате да се уверите, че имате инсталирана LTS или по-нова версия на NodeJS. Можете да я получите тук: https://nodejs.org/en/download/

След като NodeJS е инсталиран, ще искате да инсталирате редактор за код.

Препоръчваме (и ние използваме) Webstorm, който можете да получите тук с 30-дневен пробен период (не е необходима кредитна карта): https://www.jetbrains.com/webstorm/

Следващата най-добра безплатна опция вероятно би била Visual Studio Code: https://code.visualstudio.com/download

След като имате настроен редактора и папката с темата отворена в редактора, отворете терминала в IDE-то и изпълнете:

[inline-code-attrs-start title = 'Инсталирайте темата'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install
[inline-code-end]

Успешният изход ще изглежда така (можете да игнорирате предупреждения):

<div class="screenshot white-bg">
    <div class="title">Успешен изход от npm install</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-2-1-install.png" alt="Успешен изход от npm install" />
</div>

Това ще настрои зависимостите на темата за по-късните команди, които ще изпълним. Също така, експортирането зависи от това зависимостите на темата да са инсталирани, в противен случай повторният импорт няма да работи правилно.

---
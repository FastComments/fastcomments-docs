Сада када смо преузели zip фајл, извучите га у фасциклу. Ја сам преузео подразумевани `casper.zip` и распаковао га у `Downloads\casper` на Windows.

Затим, требате се уверити да имате инсталирану LTS или новију верзију NodeJS-а. Можете је преузети овдје: https://nodejs.org/en/download/

Када је NodeJS инсталиран, требате инсталирати уређивач кода.

Препоручујемо (и користимо) Webstorm, који можете добити овдје са 30-дневном пробом (није потребна кредитна картица): https://www.jetbrains.com/webstorm/

Наредна најбоља бесплатна опција би вероватно била Visual Studio Code: https://code.visualstudio.com/download

Када имате подешен уређивач и фасциклу теме отворену у уређивачу, отворите терминал у IDE-у и покрените:

[inline-code-attrs-start title = 'Инсталирање теме'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install
[inline-code-end]

Успјешни излаз ће изгледати овако (можете занемарити упозорења):

<div class="screenshot white-bg">
    <div class="title">Успјешан излаз npm install</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-2-1-install.png" alt="Успјешан излаз npm install" />
</div>

Ово ће подесити зависности теме за касније команде које ћемо покренути. Такође, извоз зависи од тога да ли су зависности теме инсталиране, у супротном поновни увоз неће исправно радити.

---
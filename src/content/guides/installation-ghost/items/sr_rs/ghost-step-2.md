---
Sada kada smo preuzeli zip fajl, raspakujte ga u folder. Preuzeo sam podrazumevani `casper.zip` i raspakovao u `Downloads\casper` na Windows.

Zatim, treba да се уверите да имате инсталирану LTS или новију верзију NodeJS-а. Možete je preuzeti ovde: https://nodejs.org/en/download/

Kada je NodeJS instaliran, требало би да инсталирате уређивач кода.

Preporučujemo (и користимо) Webstorm, који можете добити овде са 30-dnevnom пробом (није потребна кредитна картица): https://www.jetbrains.com/webstorm/

Kao sledeća najbolja besplatna opcija вероватно би био Visual Studio Code: https://code.visualstudio.com/download

Kada podesite uređivač и otvorите folder teme у uređивачу, otvorite terminal у IDE-у и pokrenite:

[inline-code-attrs-start title = 'Instalirajte temu'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install
[inline-code-end]

Uspešan izlaz komande npm install će izgledati ovako (možete zanemariti upozorenja):

<div class="screenshot white-bg">
    <div class="title">Uspešan izlaz komande npm install</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-2-1-install.png" alt="Successful npm install output" />
</div>

Ovo će podesiti zavisnosti teme за naredne komande koje ћемо покренути. Такође, експорт зависи од тога да су зависности теме инсталиране; у супротном поновни увоз неће правилно радити.

---
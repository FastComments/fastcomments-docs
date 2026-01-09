Ora che abbiamo scaricato il file zip, estrailo in una cartella. Ho scaricato il predefinito `casper.zip` ed estratto in `Downloads\casper` su Windows.

Successivamente, assicurati di avere installata la versione LTS o più recente di NodeJS. Puoi scaricarla qui: https://nodejs.org/en/download/

Una volta installato NodeJS, installa un editor di codice.

Raccomandiamo (e utilizziamo) WebStorm, che puoi ottenere qui con una prova di 30 giorni (non è richiesta la carta di credito): https://www.jetbrains.com/webstorm/

La migliore alternativa gratuita successiva è probabilmente Visual Studio Code: https://code.visualstudio.com/download

Una volta impostato l'editor e aperta la cartella del tema nell'editor, apri il terminale nell'IDE ed esegui:

[inline-code-attrs-start title = 'Installa il tema'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install
[inline-code-end]

L'output risultante sarà simile a questo (puoi ignorare gli avvisi):

<div class="screenshot white-bg">
    <div class="title">Output riuscito di npm install</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-2-1-install.png" alt="Output riuscito di npm install" />
</div>

Questo imposterà le dipendenze del tema per i comandi che eseguiremo in seguito. Inoltre, l'esportazione dipende dall'installazione delle dipendenze del tema; altrimenti la reimportazione non funzionerà correttamente.

---
### Supporto della Modalità Scura

### Modalità scura dinamica

Se la modalità scura del tuo sito è controllata aggiungendo una classe `.dark` all'elemento body, l'interfaccia di Collab Chat la rispetterà automaticamente senza richiedere una reinizializzazione. Gli stili del widget sono progettati per rispondere alla presenza di questa classe.

[inline-code-attrs-start title = 'Esempio CSS per la modalità scura'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
/* Il tuo CSS per la modalità scura */
body.dark {
    background: #1a1a1a;
    color: #ffffff;
}
[inline-code-end]

### Stilizzazione personalizzata con CSS

Puoi personalizzare l'aspetto degli evidenziamenti, delle finestre di chat e di altri elementi usando il CSS. Il widget aggiunge classi specifiche che puoi mirare nel tuo stylesheet.

Gli evidenziamenti di testo utilizzano il sistema di styling dei commenti a fumetto di FastComments, quindi qualsiasi personalizzazione applicata al widget di commenti standard influenzerà anche Collab Chat.

### Personalizzazione della barra superiore

La barra superiore mostra il numero di utenti online e il numero di discussioni. Puoi personalizzarne la posizione fornendo un elemento personalizzato come `topBarTarget`:

[inline-code-attrs-start title = 'Posizione personalizzata della barra superiore'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: document.getElementById('my-custom-header')
});
[inline-code-end]

Oppure disabilitarla completamente impostandola su `null`:

[inline-code-attrs-start title = 'Disabilitare la barra superiore'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: null
});
[inline-code-end]

### Comportamento mobile

Su schermi con larghezza inferiore a 768px, Collab Chat passa automaticamente a un layout ottimizzato per mobile. Le finestre di chat appaiono a schermo intero invece di fluttuare accanto al testo e il ritardo di selezione viene rimosso per un'interazione più immediata.

Questo comportamento è integrato e non richiede alcuna configurazione. Il widget rileva automaticamente la dimensione dello schermo e si adatta di conseguenza.

### Aspetto della finestra di chat

Le finestre di chat sono larghe 410px su desktop con una freccia di 16px che punta al testo evidenziato. Le finestre si posizionano automaticamente in base allo spazio disponibile nella viewport, usando classi di posizionamento come `to-right`, `to-left`, `to-top` e `to-bottom`.

Puoi aggiungere CSS personalizzato per regolare colori, font, spaziatura o altre proprietà visive di queste finestre. Le finestre di chat utilizzano la stessa struttura dei componenti del widget FastComments standard, quindi ereditano qualsiasi personalizzazione globale che hai applicato.

### Localizzazione

Collab Chat supporta le stesse opzioni di localizzazione del widget FastComments standard. Imposta l'opzione `locale` per visualizzare il testo dell'interfaccia in lingue diverse:

[inline-code-attrs-start title = 'Impostare la localizzazione'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    locale: 'es'  // Spagnolo
});
[inline-code-end]

FastComments supporta dozzine di lingue. L'impostazione del locale influisce su tutto il testo dell'interfaccia, inclusi prompt, pulsanti e testi dei placeholder.

### Opzioni di personalizzazione ereditate

Poiché Collab Chat estende il widget di commenti standard, eredita tutte le opzioni di personalizzazione dal widget base. Questo include classi CSS personalizzate, traduzioni personalizzate, personalizzazione degli avatar, formattazione delle date e molto altro.

Consulta la documentazione principale di personalizzazione di FastComments per l'elenco completo delle opzioni di personalizzazione disponibili.

### Lavorare con font personalizzati

Se il tuo sito utilizza font personalizzati, l'interfaccia di Collab Chat erediterà quei font dal CSS della tua pagina. Potrebbe essere necessario creare una regola di personalizzazione del widget e `@import` qualsiasi font nel CSS personalizzato in quella regola se
vuoi che la finestra di chat fluttuante utilizzi gli stessi font.
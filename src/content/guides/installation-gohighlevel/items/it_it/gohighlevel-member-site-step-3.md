Ora genereremo il tuo codice FastComments personalizzato. Usa la procedura guidata qui sotto per configurare come vuoi che FastComments funzioni sul tuo sito GoHighLevel:

[snippet id="gohighlevel-wizard"]

### Diversi tipi di box per i commenti

Puoi configurare la riga `TYPE = 'commenting'` per cambiare il prodotto utilizzato (per esempio puoi cambiarla in `live` per la chat in streaming o `collab` per la chat collab).

### Posizionare il box dei commenti dove vuoi

Supponiamo che tu voglia mettere i box dei commenti in parti specifiche della pagina e non nelle posizioni predefinite.
Cambia questa riga:

    const TARGET_ELEMENT_ID = ''; // impostalo per usare la modalità div target

In:

    const TARGET_ELEMENT_ID = 'fc_box'; // impostalo per usare la modalità div target

Poi nell'editor GHL, clicca il pulsante "code" e aggiungi il punto in cui vuoi che vadano i commenti:

[inline-code-attrs-start title = 'Div FastComments di GoHighLevel'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div
  id="fc_box"
  type="commenting"
  urlid="custom-chat-id"
></div>
[inline-code-end]

### Tipo diverso di box per commenti per pagina

Supponiamo che tu voglia permettere agli utenti di evidenziare e discutere parti di testo, o usare l'interfaccia chat in streaming invece.

Prima segui i passaggi sopra in "Posizionare il box dei commenti dove vuoi".

Nota che in quel piccolo snippet c'è `type="commenting"`.

Se vuoi abilitare la chat collab, ad esempio, cambia type in `type="collab"`.

### Mostra solo su pagine specifiche

Se non imposti `TARGET_ELEMENT_ID`, puoi invece configurare la variabile `VALID_PATTERNS`, per impostare su quali percorsi URL i commenti dovrebbero apparire. Per impostazione predefinita, verrà mostrato sulle pagine che contengono `/post` nell'URL.

### Configurazione della chat collab

Puoi dire alla chat collab di aggiungere funzionalità collaborative solo attorno all'HTML dentro una specifica area, per esempio, supponiamo che tu aggiunga il codice del footer sopra e poi aggiunga questo div nel contenuto del post/pagina per abilitare la chat collab:

[inline-code-attrs-start title = 'Chat collab con contenuto specificato'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div
  id="fc_box"
  type="collab"
  urlid="custom-chat-id"
><p>This content will have collab chat!</p></div>
[inline-code-end]

Allora l'elemento paragrafo all'interno del `<div>` avrà la chat collab abilitata, e nient'altro nella pagina. Se non metti contenuto nel `<div>` allora abiliterà la chat collab sull'intero corpo del post.
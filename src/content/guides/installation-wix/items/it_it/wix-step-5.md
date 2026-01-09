---
Successivamente, configuriamo le cose in modo che il thread dei commenti cambi in base alla pagina corrente, permettendo agli utenti di discutere il contenuto attualmente visualizzato.

Senza i passaggi seguenti, avrai solo un unico thread di commenti globale per l'intero sito - il che non è molto utile.

#### Modalità Dev

Per aggiungere questa funzionalità, dovremo entrare in ciò che Wix chiama `Dev Mode`.

Clicca sull'opzione `Dev Mode` in cima allo schermo.

<div class="screenshot white-bg">
    <div class="title">Abilita Modalità Dev</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-dev-mode.png" alt="Abilita Modalità Dev" />
</div>

#### Imposta l'ID dell'elemento

Aggiungeremo del codice personalizzato per realizzare questo, ma prima dobbiamo assegnare un ID al nuovo elemento embed in modo da potervi fare riferimento.

Chiamiamolo `fastcomments`.

Clicca sul nuovo elemento embed che abbiamo aggiunto e in dev mode in basso a destra dovresti vedere un campo ID con un valore tipo `html1`:

<div class="screenshot white-bg">
    <div class="title">Il campo ID</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-0.png" alt="Il campo ID" />
</div>

Cambia questo in `fastcomments` e premi invio:

<div class="screenshot white-bg">
    <div class="title">Imposta l'ID</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-1.png" alt="Imposta l'ID" />
</div>

Ora possiamo aggiungere il nostro codice personalizzato che dice all'area dei commenti quale pagina stiamo visualizzando.

In fondo allo schermo dovresti vedere un editor di codice come questo:

<div class="screenshot white-bg">
    <div class="title">Apri l'editor</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-open-editor.png" alt="Apri l'editor" />
</div>

Copia il codice seguente e incollalo lì:

[inline-code-attrs-start title = 'Snippet di navigazione per i commenti di Wix'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import wixLocation from 'wix-location';

$w.onReady(function () {
    function updateFastCommentsLocation() {
        try {
            const url = (wixLocation.baseUrl + '/' + wixLocation.path).replace(/,/g, '/');
            $w('#fastcomments').postMessage({'action': 'reload', 'url': url});
        } catch (err) {
            console.error('Wix -> FastComments Error', err);
        }
    }

    updateFastCommentsLocation();
    wixLocation.onChange( () => {
        updateFastCommentsLocation();
    });
});
[inline-code-end]

<div class="screenshot white-bg">
    <div class="title">Aggiungi il codice di navigazione</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-paste-code.png" alt="Aggiungi il codice di navigazione" />
</div>

---
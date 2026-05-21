The **FastComments** block supporta il single sign-on in modo che i tuoi clienti Shopify possano commentare come se stessi senza creare un account FastComments separato.

### Come funziona

Quando un visitatore che ha effettuato l'accesso al tuo negozio apre una pagina con il blocco **FastComments**:

1. Il blocco rileva l'oggetto Shopify `customer`.
2. Invia il nome e l'email del cliente a FastComments tramite una richiesta app proxy firmata.
3. FastComments crea o associa un utente identificato da `shopify-{customerId}`, così lo stesso cliente viene sempre mappato allo stesso utente FastComments tra sessioni e reinstallazioni.
4. Il loro nome appare nei commenti. Non viene loro richiesto di effettuare nuovamente l'accesso.

Se il visitatore non ha effettuato l'accesso al negozio, il blocco ricade sui commenti anonimi (o sul flusso di accesso di FastComments, a seconda della configurazione del widget).

### Disattivare SSO

SSO è attivato per impostazione predefinita per ogni blocco **FastComments**. Per disattivarlo su uno specifico blocco:

1. Apri l'editor del tema Shopify.
2. Apri il template che contiene il blocco e fai clic sul blocco per selezionarlo.
3. Deseleziona **SSO**.
4. Fai clic su **Salva**.

Disattiva SSO se vuoi che i commentatori scelgano un'identità separata per la conversazione. Ad esempio, una pagina di community interna dove il personale commenta con un nome visualizzato diverso.

### Cosa riceve FastComments

Il payload SSO inviato per ogni cliente contiene:

- Un ID utente derivato dall'ID cliente Shopify (`shopify-{customerId}`).
- L'email del cliente (usata per identificare l'utente; non visualizzata pubblicamente).
- Il nome visualizzato del cliente (usato come autore del commento).

Non vengono inviati dati di cronologia ordini, pagamenti o indirizzi. Il payload è firmato lato server; il browser del cliente non vede mai una credenziale.

### Link di accesso e disconnessione

Quando SSO è attivato, i link di accesso e disconnessione del widget dei commenti puntano a `/account/login` e `/account/logout`, le rotte standard dell'account cliente Shopify. Non c'è nulla da configurare. I link funzionano per qualsiasi negozio con gli account cliente abilitati.
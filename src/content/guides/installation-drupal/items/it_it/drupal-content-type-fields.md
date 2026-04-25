---
Per la maggior parte dei siti, il modo più semplice per aggiungere commenti è allegare il campo `FastComments` ai tipi di contenuto. Vai su `Structure > Content types > [type] > Manage fields` e aggiungi il campo.

Ogni entità che possiede il campo ottiene:

- Un **interruttore di stato** che permette agli editor di attivare o disattivare i commenti per entità.
- Un **identificatore personalizzato** opzionale, così puoi usare un ID stabile che non è legato al percorso dell'entità Drupal.

Il blocco principale `FastComments Widget` è a conoscenza di questo campo e salterà le entità che lo hanno già allegato. In questo modo puoi combinare commenti per entità con il blocco senza vedere il widget due volte nella stessa pagina.

---
FastComments ti consente di richiedere ai commentatori alla prima visita di accettare i tuoi Termini di Servizio prima di inviare un commento.

Quando abilitato:
- **Utenti anonimi** vedranno una casella di controllo TOS ogni volta che commentano
- **Utenti autenticati** vedranno la casella di controllo solo al loro primo commento, o quando aggiorni i tuoi Termini di Servizio

### Configurazione

Vai alla pagina di personalizzazione del widget e abilita la casella di controllo "Richiedi l'accettazione dei Termini di Servizio". Una volta abilitata, vedrai le seguenti opzioni:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.tos-enabled'; selector = '#tos-options'; alt='Pannello dei Termini di Servizio che mostra il selettore della modalità di testo TOS e il campo della data di ultimo aggiornamento'; title='Opzioni dei Termini di Servizio' app-screenshot-end]

- **Modalità Testo TOS**: Per impostazione predefinita, la casella di controllo visualizza "Accetto i Termini di Servizio e l'Informativa sulla Privacy" con collegamenti a entrambi i documenti. Seleziona "Personalizza il testo per lingua" per fornire il tuo testo per ogni lingua.
- **Data Ultimo Aggiornamento TOS**: Quando aggiorni i tuoi Termini di Servizio, imposta questa data. Gli utenti che hanno accettato prima di questa data saranno tenuti ad accettare nuovamente.

### Come Funziona

- Il timestamp di accettazione dei TOS è memorizzato per utente e per commento
- Quando un utente accetta i TOS, la data viene registrata sul suo profilo utente (per tenant)
- Se imposti una data "Ultimo Aggiornamento" successiva alla data di accettazione dell'utente, questi dovrà ri-accettare
- Per gli utenti anonimi che non possono essere tracciati, la casella di controllo appare su ogni invio di commento
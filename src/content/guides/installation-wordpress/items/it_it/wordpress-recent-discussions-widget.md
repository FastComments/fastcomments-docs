Il widget Recent Discussions mostra le pagine del tuo sito con l'attività di commenti più recente. È utile per mettere in evidenza le discussioni che ricevono ancora nuovi commenti, così i visitatori possono tornare nelle conversazioni attive invece di atterrare su pagine silenziose.

## Opzioni

- **Title** (opzionale): Il titolo mostrato sopra l'elenco. Il valore predefinito è "Recent Discussions".
- **Count** (opzionale): Quante discussioni mostrare. Intervallo da 1 a 50. Il valore predefinito è 20.

## Come Aggiungerlo

### Dentro un Articolo o una Pagina

Nell'editor a blocchi, aggiungi un blocco **Shortcode** e incolla:

[inline-code-attrs-start title = 'Shortcode Recent Discussions'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_recent_discussions count="20"]
[inline-code-end]

L'attributo `count` accetta qualsiasi valore tra 1 e 50.

### In una Sidebar o nel Footer (Temi Classici)

Vai su **Aspetto > Widget** nell'amministrazione di WordPress. Dall'inseritore di blocchi, cerca "FastComments" e scegli **FastComments: Recent Discussions**. Trascinalo in un'area sidebar, header o footer, poi configura il titolo e il conteggio dal pannello del widget.

### In un Tema a Blocchi (Full Site Editing)

Apri l'**Editor del Sito** sotto **Aspetto > Editor**. Naviga fino alla parte del template in cui il widget dovrebbe apparire, inserisci un blocco **Legacy Widget** e seleziona **FastComments: Recent Discussions** dal menu a tendina.

## Risoluzione dei Problemi

Il widget viene visualizzato solo dopo che la configurazione di FastComments è stata completata e un tenant ID è stato salvato. Se l'area del widget è vuota, completa la configurazione sotto **FastComments** nell'amministrazione di WordPress e ricarica la pagina.

Se l'ordinamento delle discussioni sembra non aggiornato, verifica che le pagine sottostanti abbiano completato la sincronizzazione nella dashboard di FastComments. Il widget legge dati in tempo reale, quindi i commenti appena importati potrebbero richiedere un momento per apparire.
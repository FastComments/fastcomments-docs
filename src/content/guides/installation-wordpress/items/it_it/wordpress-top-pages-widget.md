Il widget Top Pages mostra le pagine con più commenti sul tuo sito. È utile per mettere in evidenza i contenuti più coinvolgenti ai nuovi visitatori e aumentare il tempo sul sito.

## Opzioni

- **Titolo** (opzionale): L'intestazione mostrata sopra l'elenco. Per impostazione predefinita è "Top Pages".

Il widget Top Pages sceglie il proprio layout in base ai dati disponibili e non accetta un attributo count.

## Come aggiungerlo

### All'interno di un post o di una pagina

Nell'editor a blocchi, aggiungi un blocco **Shortcode** e incolla:

[inline-code-attrs-start title = 'Shortcode Pagine principali'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_top_pages]
[inline-code-end]

### In una barra laterale o nel footer (temi classici)

Vai su **Aspetto > Widget** nella tua amministrazione WordPress. Dall'inseritore di blocchi, cerca "FastComments" e scegli **FastComments: Top Pages**. Trascinalo in una barra laterale, nell'intestazione o nell'area del footer, quindi imposta il titolo dal pannello del widget.

### In un tema a blocchi (Editor del sito)

Apri l'**Editor del sito** sotto **Aspetto > Editor**. Naviga nella parte del template in cui il widget dovrebbe apparire, inserisci un blocco **Legacy Widget** e seleziona **FastComments: Top Pages** dal menu a tendina.

## Risoluzione dei problemi

Il widget viene visualizzato solo dopo che la configurazione di FastComments è completa e un tenant ID è stato memorizzato. Se l'area del widget è vuota, completa la configurazione sotto **FastComments** nell'amministrazione di WordPress e ricarica la pagina.
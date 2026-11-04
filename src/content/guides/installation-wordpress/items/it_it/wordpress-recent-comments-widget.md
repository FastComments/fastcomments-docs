Il widget Commenti Recenti mostra i commenti più recenti pubblicati su tutto il tuo sito. È utile nelle sidebar, nei footer o in qualsiasi punto in cui desideri mettere in evidenza l'attività recente per incoraggiare ulteriori letture.

## Opzioni

- **Titolo** (opzionale): L'intestazione mostrata sopra l'elenco. Predefinito: "Commenti Recenti".
- **Conteggio** (opzionale): Quanti commenti mostrare. Intervallo da 1 a 50. Predefinito: 5.

## Come Aggiungerlo

### Dentro un Articolo o una Pagina

Nell'editor a blocchi, aggiungi un blocco **Shortcode** e incolla:

[inline-code-attrs-start title = 'Shortcode Commenti Recenti'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_recent_comments count="5"]
[inline-code-end]

L'attributo `count` accetta qualsiasi valore tra 1 e 50.

### In una Sidebar o Footer (Temi Classici)

Vai su **Aspetto > Widget** nell'amministrazione di WordPress. Dall'inseritore di blocchi, cerca "FastComments" e scegli **FastComments: Commenti Recenti**. Trascinalo in una sidebar, nell'header o nell'area del footer, quindi configura il titolo e il conteggio dal pannello del widget.

### In un Tema a Blocchi (Full Site Editing)

Apri l'**Editor del Sito** sotto **Aspetto > Editor**. Naviga fino alla parte del template in cui il widget dovrebbe apparire, inserisci un blocco **Legacy Widget** e seleziona **FastComments: Commenti Recenti** dal menu a tendina.

## Risoluzione dei Problemi

Il widget viene visualizzato solo dopo che la configurazione di FastComments è stata completata e un tenant ID è stato memorizzato. Se l'area del widget è vuota, completa la configurazione sotto **FastComments** nell'amministrazione di WordPress e ricarica la pagina.
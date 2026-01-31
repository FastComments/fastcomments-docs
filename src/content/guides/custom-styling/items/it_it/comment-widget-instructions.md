## Come personalizzare gli stili del widget dei commenti

Puoi personalizzare lo stile del widget dei commenti in due modi:

### Opzione 1: Tramite il parametro customCSS

Passa il tuo CSS personalizzato come stringa al parametro `customCSS` durante l'inizializzazione del widget:

```javascript
window.fcConfigs = [{
    target: '#fastcomments-widget',
    tenantId: 'your-tenant-id',
    customCSS: `
        .fast-comments .comment {
            background-color: #f0f0f0 !important;
            border-radius: 8px !important;
        }
    `
}];
```

### Opzione 2: Tramite la Dashboard di amministrazione

1. Vai alla [pagina di personalizzazione del widget](https://fastcomments.com/auth/my-account/customize-widget) nella tua dashboard di amministrazione
2. Scorri fino alla sezione "CSS personalizzato" sotto "Avanzate"
3. Inserisci il tuo CSS personalizzato
4. Fai clic su "Salva"

Il tuo CSS personalizzato verrà applicato a tutti i widget dei commenti sul tuo sito.

## Suggerimenti

- Usa `!important` per sovrascrivere gli stili predefiniti, se necessario
- Usa selettori specifici per evitare di influenzare altre parti del tuo sito
- Testa il tuo CSS in diversi browser per verificarne la compatibilità
- Il widget usa CSS standard - non sono necessari preprocessori speciali
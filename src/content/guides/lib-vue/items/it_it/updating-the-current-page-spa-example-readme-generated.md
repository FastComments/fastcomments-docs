In FastComments chiamiamo l'id dell'articolo, o della pagina a cui i commenti sono legati, URL ID poiché può essere un url o un ID.
Definisci l'URL ID nel seguente modo. Il componente osserva le modifiche nell'oggetto config e si ricaricherà, quindi puoi semplicemente aggiornare le impostazioni "url" e "urlId".

Vedi un esempio completo e funzionante [qui](https://github.com/FastComments/fastcomments-vue/blob/master/dev/serve-pagination.ts).

Esegui l'esempio di paginazione con:

```
npm run serve-pagination
```

```vue
<fast-comments-vue v-bind:config="{tenantId: 'demo', url: 'https://example.com', urlId: 'some-page-id'}" />
```

### Regione account (ATTENZIONE: clienti UE)

Se il tuo account si trova nell'UE, imposta `region = 'eu'` nella configurazione del widget, per esempio:

```vue
<fast-comments-vue v-bind:config="{tenantId: 'demo', url: 'https://example.com', urlId: 'some-page-id', region: 'eu'}" />
```

Altrimenti, non è necessario definire `region`.
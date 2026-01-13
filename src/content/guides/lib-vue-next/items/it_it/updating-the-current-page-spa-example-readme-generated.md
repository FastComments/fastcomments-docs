---
In FastComments chiamiamo l'id dell'articolo, o della pagina a cui i commenti sono legati, l'URL ID poiché può essere un url o un ID.
Definisci l'URL ID nel modo seguente. Il componente osserva le modifiche nell'oggetto config, e si ricaricherà, quindi puoi aggiornare l'URL ID.

```vue
<FastComments v-bind:config="{tenantId: 'demo', url: 'https://example.com/somepage', urlId: 'some-page-id'}" />
```

### Regione dell'account (ATTENZIONE: Clienti UE)

Se il tuo account si trova nell'UE, imposta `region = 'eu'` nella configurazione del widget, per esempio:

```vue
<FastComments v-bind:config="{tenantId: 'demo', url: 'https://example.com/somepage', urlId: 'some-page-id', region: 'eu'}" />
```

Altrimenti, non è necessario definire `region`.
---
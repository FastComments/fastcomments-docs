U FastComments-u nazivamo ID članka, odnosno stranice s kojom su komentari povezani, URL ID-jem jer može biti URL ili ID.
Definišite URL ID na sljedeći način. Komponenta prati promjene u objektu config i ponovo će učitati, tako da možete ažurirati URL ID.

```vue
<FastComments v-bind:config="{tenantId: 'demo', url: 'https://example.com/somepage', urlId: 'some-page-id'}" />
```

### Regija naloga (PAŽNJA: EU korisnici)

Ako je vaš nalog smješten u EU, postavite `region = 'eu'` u konfiguraciji widgeta, na primjer:

```vue
<FastComments v-bind:config="{tenantId: 'demo', url: 'https://example.com/somepage', urlId: 'some-page-id', region: 'eu'}" />
```

U suprotnom, ne morate definirati `region`.
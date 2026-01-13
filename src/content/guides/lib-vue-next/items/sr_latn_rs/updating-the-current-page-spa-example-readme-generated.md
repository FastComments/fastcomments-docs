U FastComments nazivamo ID članka, odnosno stranicu na koju su komentari vezani, URL ID-jem, jer može biti URL ili ID.
Definišite URL ID na sledeći način. Komponenta prati promene u config objektu i ponovo će se učitati, tako da možete ažurirati URL ID.

```vue
<FastComments v-bind:config="{tenantId: 'demo', url: 'https://example.com/somepage', urlId: 'some-page-id'}" />
```

### Region naloga (PAŽNJA: korisnici iz EU)

Ako se vaš nalog nalazi u EU, postavite `region = 'eu'` u konfiguraciji widgeta, na primer:

```vue
<FastComments v-bind:config="{tenantId: 'demo', url: 'https://example.com/somepage', urlId: 'some-page-id', region: 'eu'}" />
```

U suprotnom, ne morate definisati `region`.
---
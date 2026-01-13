U FastCommentsu nazivamo article id, odnosno stranicu s kojom su komentari povezani, URL ID, jer to može biti url ili ID.
Definirajte URL ID na sljedeći način. Komponenta prati promjene u objektu config i ponovno će se učitati, tako da možete ažurirati URL ID.

```vue
<FastComments v-bind:config="{tenantId: 'demo', url: 'https://example.com/somepage', urlId: 'some-page-id'}" />
```

### Regija računa (PAŽNJA: EU korisnici)

Ako se vaš račun nalazi u EU, postavite `region = 'eu'` u konfiguraciji widgeta, na primjer:

```vue
<FastComments v-bind:config="{tenantId: 'demo', url: 'https://example.com/somepage', urlId: 'some-page-id', region: 'eu'}" />
```

Inače, ne morate definirati `region`.
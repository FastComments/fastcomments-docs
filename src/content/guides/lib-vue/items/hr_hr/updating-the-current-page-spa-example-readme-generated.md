U FastCommentsu nazivamo identifikator članka, odnosno stranice na koju su pripisani komentari, URL ID-om jer može biti URL ili ID.
Definirajte URL ID na sljedeći način. Komponenta promatra promjene u objektu config i ponovno će se učitati, tako da možete jednostavno ažurirati postavke "url" i "urlId".

Pogledajte potpuno radni primjer [ovdje](https://github.com/FastComments/fastcomments-vue/blob/master/dev/serve-pagination.ts).

Pokrenite primjer paginacije pomoću:

```
npm run serve-pagination
```

```vue
<fast-comments-vue v-bind:config="{tenantId: 'demo', url: 'https://example.com', urlId: 'some-page-id'}" />
```

### Regija računa (PAŽNJA: kupci iz EU)

Ako je vaš račun smješten u EU, postavite `region = 'eu'` u konfiguraciji widgeta, na primjer:

```vue
<fast-comments-vue v-bind:config="{tenantId: 'demo', url: 'https://example.com', urlId: 'some-page-id', region: 'eu'}" />
```

U suprotnom, ne morate definirati `region`.
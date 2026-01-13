U FastComments-u nazivamo ID članka, odnosno stranice na koju su komentari vezani, URL ID-jem jer može biti url ili ID.
Definišite URL ID na sljedeći način. Komponenta nadgleda promjene u config objektu i ponovo će se učitati, tako da možete jednostavno ažurirati "url" i "urlId" postavke.

Pogledajte kompletan radni primjer [ovdje](https://github.com/FastComments/fastcomments-vue/blob/master/dev/serve-pagination.ts).

Pokrenite primjer paginacije putem:

```
npm run serve-pagination
```

```vue
<fast-comments-vue v-bind:config="{tenantId: 'demo', url: 'https://example.com', urlId: 'some-page-id'}" />
```

### Regija naloga (PAŽNJA: EU korisnici)

Ako je vaš nalog smješten u EU, postavite `region = 'eu'` u konfiguraciji widgeta, na primjer:

```vue
<fast-comments-vue v-bind:config="{tenantId: 'demo', url: 'https://example.com', urlId: 'some-page-id', region: 'eu'}" />
```

U suprotnom, ne morate definirati `region`.
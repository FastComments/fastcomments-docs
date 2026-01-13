U FastComments nazivamo ID članka, odnosno stranice na koju su komentari vezani, URL ID jer to može biti url ili ID.
Definišite URL ID na sledeći način. Komponenta nadgleda promene u objektu config i ponovo će se učitati, tako da možete samo ažurirati podešavanja "url" i "urlId".

Pogledajte kompletan radni primer [ovde](https://github.com/FastComments/fastcomments-vue/blob/master/dev/serve-pagination.ts).

Pokrenite primer paginacije pomoću:

```
npm run serve-pagination
```

```vue
<fast-comments-vue v-bind:config="{tenantId: 'demo', url: 'https://example.com', urlId: 'some-page-id'}" />
```

### Account Region (PAŽNJA: kupci iz EU)

Ako je vaš nalog smešten u EU, postavite `region = 'eu'` u konfiguraciji widgeta, na primer:

```vue
<fast-comments-vue v-bind:config="{tenantId: 'demo', url: 'https://example.com', urlId: 'some-page-id', region: 'eu'}" />
```

U suprotnom, ne morate definisati `region`.
---
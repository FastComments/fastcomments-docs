V FastComments id članka ali strani, na katero so pripeti komentarji, imenujemo URL ID, saj je lahko URL ali ID.
Določite URL ID na naslednji način. Komponenta spremlja spremembe v objektu config in se bo ponovno naložila, zato lahko preprosto posodobite nastavitve "url" in "urlId".

Oglejte si celoten delujoč primer [tukaj](https://github.com/FastComments/fastcomments-vue/blob/master/dev/serve-pagination.ts).

Zaženite primer paginacije z:

```
npm run serve-pagination
```

```vue
<fast-comments-vue v-bind:config="{tenantId: 'demo', url: 'https://example.com', urlId: 'some-page-id'}" />
```

### Account Region (ATTENTION: EU Customers)

Če je vaš račun v EU, nastavite `region = 'eu'` v konfiguraciji vtičnika, na primer:

```vue
<fast-comments-vue v-bind:config="{tenantId: 'demo', url: 'https://example.com', urlId: 'some-page-id', region: 'eu'}" />
```

V nasprotnem primeru vam ni treba določiti `region`.
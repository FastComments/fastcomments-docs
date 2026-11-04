V FastComments članku ID oziroma stran, na katero so povezani komentarji, imenujemo URL ID, saj je lahko URL ali ID.

URL ID določite na naslednji način. Komponenta spremlja spremembe v config objektu in se bo ponovno naložila, zato lahko preprosto posodobite nastavitvi "url" in "urlId".

Glejte celoten delujoč primer [tukaj](https://github.com/FastComments/fastcomments-vue/blob/master/dev/serve-pagination.ts).

Zaženite primer paginacije z:

```
npm run serve-pagination
```

```vue
<fast-comments-vue v-bind:config="{tenantId: 'demo', url: 'https://example.com', urlId: 'some-page-id'}" />
```

### Regija računa (POZOR: stranke iz EU)

Če je vaš račun lociran v EU, nastavite `region = 'eu'` v konfiguraciji pripomočka, na primer:

```vue
<fast-comments-vue v-bind:config="{tenantId: 'demo', url: 'https://example.com', urlId: 'some-page-id', region: 'eu'}" />
```

Sicer ni treba določiti `region`.
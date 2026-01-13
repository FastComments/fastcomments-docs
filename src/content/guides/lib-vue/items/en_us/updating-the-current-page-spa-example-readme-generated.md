In FastComments, we call the article ID, or the page the comments are tied to, the URL ID, since it can be a URL or an ID.
Define the URL ID as follows. The component watches for changes in the config object and will reload, so you can simply update the "url" and "urlId" settings.

See a full working example [here](https://github.com/FastComments/fastcomments-vue/blob/master/dev/serve-pagination.ts).

Run the pagination example via:

```
npm run serve-pagination
```

```vue
<fast-comments-vue v-bind:config="{tenantId: 'demo', url: 'https://example.com', urlId: 'some-page-id'}" />
```

### Account Region (ATTENTION: EU Customers)

If your account is located in the EU, set `region = 'eu'` in the widget configuration, for example:

```vue
<fast-comments-vue v-bind:config="{tenantId: 'demo', url: 'https://example.com', urlId: 'some-page-id', region: 'eu'}" />
```

Otherwise, you do not have to define `region`.
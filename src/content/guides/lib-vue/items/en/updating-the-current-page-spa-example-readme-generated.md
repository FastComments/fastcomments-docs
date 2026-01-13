In FastComments we call the article id, or page the comments get tied to, the URL ID as it can be a url or an ID.
Define the URL ID in the following manner. The component watches for changes in config object, and will reload, so you can just update the "url" and "urlId" settings.

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
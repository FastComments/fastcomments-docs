Στο FastComments, την ταυτότητα του άρθρου ή της σελίδας στην οποία συνδέονται τα σχόλια την ονομάζουμε URL ID, καθώς μπορεί να είναι είτε url είτε ID.
Ορίστε το URL ID με τον ακόλουθο τρόπο. Το component παρακολουθεί αλλαγές στο αντικείμενο config και θα ξαναφορτώσει, οπότε μπορείτε απλά να ενημερώσετε τις ρυθμίσεις "url" και "urlId".

Δείτε ένα πλήρες παράδειγμα λειτουργίας [here](https://github.com/FastComments/fastcomments-vue/blob/master/dev/serve-pagination.ts).

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
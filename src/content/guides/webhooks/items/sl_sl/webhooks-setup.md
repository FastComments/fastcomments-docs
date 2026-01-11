Upoštevajte enake korake za `localhost`, kot bi jih za produkcijsko okolje. Prepričajte se, da imate nastavljene produkcijske domene in API Secrets.

Najprej odprite [Webhooks admin](https://fastcomments.com/auth/my-account/manage-data/webhooks). Do tega dostopate prek Manage Data -> Webhooks.

Stran s konfiguracijo se prikaže tako:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; title='Webhooks Configuration' app-screenshot-end]

Na tej strani lahko določite končne točke za vsako vrsto dogodka komentarja.

Za vsako vrsto dogodka ne pozabite klikniti Send Test Payload, da zagotovite, da je vaša integracija pravilno nastavljena. Podrobnosti najdete v naslednjem razdelku, "Testing", za podrobnosti.
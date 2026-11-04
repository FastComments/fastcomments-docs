Za `localhost` sledite istim korakom kot pri produkciji. Poskrbite, da imate nastavljene produkcijske domene in API skrivnosti.

Najprej odprite [Webhooks admin](https://fastcomments.com/auth/my-account/manage-data/webhooks). Dostopen je prek Manage Data -> Webhooks.

Stran prikaže vse webhook-e na vašem računu:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; alt='Stran Webhooks admin s seznamom webhook-ov, ki prikazuje URL, dogodek, domeno, metodo, status in število čakajočih dogodkov'; title='Seznam webhook-ov'; cacheBuster = 'v4' app-screenshot-end]

Kliknite **New Webhook**, da dodate novega. Vsak webhook ima URL, en dogodek komentarja (ustvarjen, posodobljen ali izbrisan), domeno in HTTP metodo:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks/new'; selector = '.content'; alt='Obrazec za nov webhook s polji URL, dogodek, domena in HTTP metoda ter gumbom Send Test Payload'; title='Nov webhook'; cacheBuster = 'v4' app-screenshot-end]

Vsak webhook se dostavlja neodvisno. Isti dogodek lahko pošljete na več končnih točk, webhook, določen na **All Domains**, pa prejema komentarje z vseh domen, tudi če obstaja webhook za določeno domeno za isti dogodek. Istega URL-ja, dogodka in domene ni mogoče dodati dvakrat.

Pred shranjevanjem kliknite **Send Test Payload**, da preverite, ali končna točka sprejema podpisan zahtevek. Podrobnosti si oglejte v naslednjem razdelku »Testiranje«.

S seznama lahko webhook uredite, onemogočite, ponovno omogočite ali izbrišete. Onemogočanje ohrani čakajoče dogodke, dokler webhook ni ponovno omogočen; brisanje jih odstrani.

Webhook-e je mogoče ustvariti tudi prek API-ja, na primer prek Zapierja. Ti se prikažejo v istem seznamu z virom **API**. Glejte Upravljanje webhook-ov prek API-ja.
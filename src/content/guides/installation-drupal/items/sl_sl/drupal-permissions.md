Modul doda tri Drupal dovoljenja, ki jih lahko dodelite na vlogo pod `People > Permissions`.

- **Administer FastComments** - Dostop do obrazca za nastavitve FastComments na `/admin/config/content/fastcomments`.
- **View FastComments** - Zahtevano za ogled pripomočka za komentarje. Brez tega dovoljenja se pripomoček ne prikaže.
- **Toggle FastComments** - Omogoča uporabnikom, da vključijo ali izključijo komentarje na ravni posamezne entitete z uporabo pripomočka polja.

By default, only users with the `administer site configuration` permission can change FastComments settings. Grant `View FastComments` to anonymous and authenticated users if you want visitors to see the widget.
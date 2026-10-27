Иако се зове Domain Migration tool, може се користити и за миграцију коментара преко појединачних страница.

Једноставно унесите старе и нове [urlId](/guide-customizations-and-configuration.html#url-id) вредности у поља за унос `from` и `to`.

### Миграција страница преко API‑ја

Да бисте мигрирали више страница одједном, користите [`POST /api/v1/pages/migrations`](/guide-api.html#pages-migrations-post). Прихвата до 100 парова `from` и `to` по захтеву и извршава их као групни посао.

Када је `type` постављен на `urlId`, премешта се само страница чији је `urlId` тачно `from`, тако да `post-1` не одговара `post10`. Алатка на контролној табли не поседује ову опцију.

Свака API миграција може се поништити у року од 90 дана помоћу [`POST /api/v1/pages/migrations/:undoId/undo`](/guide-api.html#pages-migrations-undo-post).
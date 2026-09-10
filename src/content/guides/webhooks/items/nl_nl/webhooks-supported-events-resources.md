FastComments ondersteunt webhooks alleen voor de Comment resource.

We ondersteunen webhooks voor het aanmaken, verwijderen en bijwerken van commentaar.

Elk van deze wordt beschouwd als een afzonderlijk evenement in ons systeem en heeft daarom verschillende semantiek
en structuren voor de webhook‑evenementen.

Een willekeurig aantal eindpunten kan zich abonneren op hetzelfde evenement, via het dashboard of via de API
(zie Managing Webhooks via de API). Elke webhook wordt onafhankelijk afgeleverd.
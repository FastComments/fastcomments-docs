---
FastComments ondersteunt webhooks alleen voor de Comment resource.

We ondersteunen webhooks voor het aanmaken, verwijderen en bijwerken van opmerkingen.

Elk van deze wordt beschouwd als een afzonderlijk evenement in ons systeem en heeft daarom verschillende semantiek
en structuren voor de webhook‑evenementen.

Een willekeurig aantal eindpunten kan zich abonneren op hetzelfde evenement: één webhook per domein kan worden geconfigureerd in
het dashboard, en verdere abonnementen kunnen worden aangemaakt via de API (zie Webhooks beheren via de API).

---
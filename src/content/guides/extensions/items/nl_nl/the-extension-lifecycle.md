Het script voor elke extensie wordt opgehaald en aangeroepen voordat de commentaar-widget begint met het ophalen van de eerste set reacties en het renderen van de UI.

Bij het initiële laden wordt de volgende data aan het extensie-object toegevoegd:

- `config` - Een verwijzing naar het `config` object.
- `translations` - Een verwijzing naar het `translations` object.
- `commentsById` - Een verwijzing naar alle reacties per id.
- `root` - Een verwijzing naar de root DOM-node.

Extensies moeten de gewenste functies overschrijven, die de commentaar-widget op de juiste momenten zal aanroepen.
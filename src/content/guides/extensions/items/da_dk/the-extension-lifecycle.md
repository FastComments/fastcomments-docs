Scriptet for hver udvidelse hentes og køres, før kommentarboksen begynder at hente det første sæt kommentarer og gengive brugergrænsefladen.

Ved første indlæsning vil følgende data blive tilknyttet udvidelsesobjektet:

- `config` - En reference til `config`-objektet.
- `translations` - En reference til `translations`-objektet.
- `commentsById` - En reference til alle kommentarer efter id.
- `root` - En reference til rod-DOM-noden.

Udvidelser bør overskrive de ønskede funktioner, som kommentarboksen vil kalde på de relevante tidspunkter.
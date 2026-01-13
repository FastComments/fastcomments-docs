Denne vejledning indeholder den komplette standard-CSS, som bruges af FastComments' kommentar-widget (v2), samt nogle instruktioner til at tilsidesætte stilarterne.

At forstå standard-CSS'en gør det muligt for dig at:

- **Tilpas udseendet** ved at tilsidesætte specifikke stilarter
- **Fejlfind stylingproblemer** ved at se, hvilke klasser og selectorer der er tilgængelige
- **Byg brugerdefinerede temaer** der fungerer med widgetens struktur
- **Brug med AI-assistenter** til at generere brugerdefinerede CSS-ændringer.

## Sådan tilsidesætter du stilarter

Måden, du tilsidesætter stilarter på, varierer afhængigt af widgetten. For kommentars-widgetten skal du bruge konfigurationsparameteren `customCSS` for at sende CSS'en ind i `iframe`-en, eller angive CSS'en på Tilpasningssiden i administrationsdashboardet, som vil servere CSS'en fra vores CDN.

---
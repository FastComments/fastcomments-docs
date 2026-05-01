Widgeten "Seneste diskussioner" viser de sider på dit websted med den mest nylige kommentaraktivitet. Den er nyttig til at fremhæve tråde, der stadig får nyt indhold, så besøgende kan hoppe ind i aktive samtaler i stedet for at lande på stille sider.

## Indstillinger

- **Title** (valgfri): Overskriften vist over listen. Standard er "Seneste diskussioner".
- **Count** (valgfri): Hvor mange diskussioner der vises. Interval 1 til 50. Standard er 20.

## Sådan tilføjer du den

### I et indlæg eller en side

I blokeditoren skal du tilføje en **Kortkode**-blok og indsætte:

[inline-code-attrs-start title = 'Kortkode: Seneste diskussioner'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_recent_discussions count="20"]
[inline-code-end]

`count`-attributten accepterer enhver værdi mellem 1 og 50.

### I en sidebar eller footer (klassiske temaer)

Gå til **Udseende > Widgets** i din WordPress-administration. Fra blokindsætteren skal du søge efter "FastComments" og vælge **FastComments: Seneste diskussioner**. Træk den ind i en sidebar, header eller footer-område, og konfigurer derefter titel og antal fra widget-panelet.

### I et bloktema (Fuld webstedsredigering)

Åbn **Site Editor** under **Udseende > Editor**. Naviger til den skabelondel, hvor widgeten skal vises, indsæt en **Legacy Widget**-blok, og vælg **FastComments: Seneste diskussioner** fra dropdown-menuen.

## Fejlfinding

Widgeten bliver kun gengivet, efter at FastComments-opsætningen er fuldført, og en tenant ID er gemt. Hvis widget-området er tomt, fuldfør opsætningen under **FastComments** i WordPress-administrationen og genindlæs siden.

Hvis diskussionsrækkefølgen virker forældet, skal du kontrollere, at de underliggende sider er færdigsynkroniseret i FastComments-dashboardet. Widgeten læser live-data, så nyligt importerede kommentarer kan tage et øjeblik at blive synlige.
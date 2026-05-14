#### Gå til LTI 1.3-konfiguration

Log ind på FastComments og gå til <a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">din LTI 1.3-konfigurationsside</a>.

Hvis din konto endnu ikke har LTI-adgang, vil du se "LTI not enabled for this account" - kontakt support for at få det aktiveret på din plan.

#### Vælg en platform (valgfrit)

Under **Generer en dynamisk registrerings-URL**, brug **Platform**-rullemenuen til at fortælle FastComments, hvilket LMS du forbinder:

- D2L Brightspace
- Moodle
- Blackboard Learn
- Sakai
- Schoology
- Anden LTI 1.3-platform

Du kan også lade den stå på **Auto-detect**. Platformen hentes fra dit LMS's openid-configuration under registreringen; rullemenuen sætter kun display-etiketten for den resulterende konfiguration.

#### Generer URL

Klik på **Generer URL**. FastComments opretter et engangs-registreringstoken og viser dig en URL, der ser sådan ud:

`https://fastcomments.com/lti/v1p3/register/<long-token>`

Kopiér den. Denne URL:

- Er til **engangsbrug** - når dit LMS kalder den med succes, forbruges tokenet.
- Udløber efter **30 minutter**, hvis den ikke bruges.
- Bør holdes privat - enhver med URL'en kan registrere et værktøj på din tenant inden for de 30 minutter.

#### Eksisterende konfigurationer

Når en registrering er gennemført med succes, vises den nye konfiguration i tabellen **Existing Configurations** på samme side, med dens Platform, Udsteder, Client ID og Status. Du kan slette konfigurationer fra denne tabel, hvis du nogensinde får brug for at afregistrere.
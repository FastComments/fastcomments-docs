**Template ID:** `gaslight_detector`

Gaslight Detector overvåger kommentarredigeringer, der omskriver historien midt i en samtale – den slags hvor en forfatter ændrer betydningen af en tidligere kommentar efter at svar er blevet skrevet, hvilket får efterfølgende svar til at se ude af kontekst eller forkerte ud. Når agenten beslutter, at en redigering krydser denne grænse, gendanner den den oprindelige tekst og sender forfatteren en DM for at forklare.

Dette er en skabelon med højere risiko, fordi den ændrer brugerindhold. Kør den i [dry-run](#dry-run-mode) længere, end du ville køre en skrivebeskyttet skabelon, og placer `edit_comment` bag [godkendelse](#approval-workflow), indtil du stoler på modellens vurdering af din trafik.

### Udløsere

- **Comment edited** (`COMMENT_EDIT`) - agenten sammenligner den nye og den tidligere tekst og afgør, om redigeringen forvrænger svar, der allerede findes.

Se [Trigger: Comment Edited](#trigger-comment-edit) for den fulde payload, inklusive den tidligere kommentartekst og antal svar på tidspunktet for redigeringen.

### Tilladte værktøjer

- [`edit_comment`](#tool-edit-comment) - bruges til at gendanne den oprindelige tekst, når redigeringen vurderes til at være gaslighting.
- [`warn_user`](#tool-warn-user) - udsteder en blød advarsel, som brugeren ser ved næste besøg.
- [`send_dm`](#tools-overview) - forklaringskanalen; brugeren får en direkte besked, der beskriver, hvorfor deres redigering blev rullet tilbage.

Den kan ikke udelukke, markere som spam, stemme eller poste nye kommentarer - overfladen er bevidst snæver.

### Anbefalede tilføjelser før produktion

- **Placer `edit_comment` bag [godkendelse](#approval-workflow).** Tilbageførsel af en kommentar er synlig for forfatteren og for alle, der har set den rettede version, så en falsk positiv er pinlig. Hold godkendelser tændt, indtil dry-run viser, at agenten er konsekvent.
- **Stram prompten med hvad der tæller som gaslighting på dit site.** Standardprompten er kort med vilje. Giv modellen konkrete eksempler - "at vende et ja/nej-pålæg", "slette et tal som svar henviser til", "tilføje en fjendtlig sætning efter at svar er blevet postet" - og eksplicitte ikke-eksempler som rettelse af stavefejl, oprydning af formatering eller tilføjelse af kilder.
- **Brug antal svar fra triggerkonteksten.** Redigeringer af kommentarer uden svar kan ikke forvride en samtale; prompten bør instruere modellen i at springe disse over.
- **Sæt flueben ved "Include commenter's trust factor, account age, ban history, and recent comments"** i [Context Options](#context-options). Modellen er langt mindre aggressiv, når den kan se en langtidsgodtroende konto.
- **Overvej et kort redigerings-venstrevindue i prompten.** Mange redigeringer inden for de første 30–60 sekunder er rettelser af slåfejl; instruer modellen i at ignorere så hurtige redigeringer.

### Anbefalet dry-run-periode

Kør i mindst to uger med reel trafik i [dry-run](#dry-run-mode), før du skifter til Enabled, og gennemgå hver markeret redigering i den periode. Brug [Test Runs (Replays)](#test-runs-replays) til at afspille de sidste 30 dages redigeringer mod agenten, før du går live.
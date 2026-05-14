Når FastComments er registreret med dit LMS, tilføjer undervisere det til kurser på samme måde, som de tilføjer ethvert andet LTI-eksternt værktøj.

#### D2L Brightspace

I et kursus' indholdsområde:

1. Klik **Tilføj eksisterende aktiviteter** > **Eksterne læringsværktøjer**.
2. Vælg **FastComments** fra listen.
3. Værktøjet vises som et emne i indholdsområdet. Åbn det én gang for at initialisere kommentartråden for den ressource.

#### Moodle

I et kursus:

1. Slå **Redigeringstilstand** til.
2. I den sektion, hvor du vil have kommentarer, klik **Tilføj en aktivitet eller ressource**.
3. Vælg **FastComments** i aktivitetsvælgeren.
4. Gem. Studerende ser kommentartråden indlejret i sektionen.

#### Blackboard Learn

I et kursus:

1. Gå til et indholdsområde.
2. Klik **Opret indhold** > **FastComments** (under "Learning Tools").
3. Angiv et navn og send.

#### Sakai

Webstedets administratorer tilføjer værktøjet via **Webstedsinfo** > **Administrer værktøjer** > rul til **Eksterne værktøjer** > vælg **FastComments** > **Fortsæt**.

#### Hvordan tråde er afgrænset

FastComments opretter en separat kommentartråd per **(LMS-forekomst, kursus, ressource-link)**. Det betyder:

- To forskellige kurser i samme LMS får separate tråde, selv hvis de bruger det samme værktøjsnavn.
- Det samme FastComments-værktøj, der bruges to steder i et kursus, opretter to tråde.
- To forskellige Brightspace-installationer under samme FastComments-konto får separate tråde - LMS-hostnavnet er en del af trådidentifikatoren.

#### SSO

Studerende ser ikke en login-skærm. LMS'et sender deres identitet (navn, e-mail, avatar, rolle) til FastComments via LTI-launch, og FastComments logger dem ind automatisk. Deres kommentarer knyttes til deres LMS-konto.

Brugere med LMS-rollerne **Instructor** eller **Administrator** bliver automatisk mappet til FastComments-moderator-/administratorroller for tråden.
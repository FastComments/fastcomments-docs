**Gebruik je Moodle?** We publiceren ook een speciale Moodle-plug-in voor FastComments met een nauwere integratie dan LTI 1.3 (hooks voor synchronisatie van cijfers, uitgebreidere activiteitrapportage, native Moodle-instellingen-UI). Zie de <a href="/guide-installation-moodle.html" target="_blank">Moodle plug-in installatiehandleiding</a>. De LTI 1.3 flow hieronder is de juiste keuze als je een enkele registratie wilt die ook andere LMSes dekt, of als je Moodle-beheerder geen plug-ins van derden wil installeren.

Moodle 4.0+ ondersteunt LTI 1.3 Dynamic Registration via de External Tool-plug-in.

#### Open het Toolbeheer-scherm

1. Meld je aan bij Moodle als sitebeheerder.
2. Navigeer naar **Site administration** > **Plugins** > **Activity modules** > **External tool** > **Manage tools**.

#### Plak de URL

Je ziet een kaart met het label **Tool URL**. Plak de FastComments-registratie-URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">get it here</a>) in het tekstveld en klik op **Add LTI Advantage**.

Moodle opent een registratiescherm waarin de identiteit van de tool en de gevraagde machtigingen worden weergegeven. Controleer dit en klik op **Activate** (of **Register**, afhankelijk van de Moodle-versie).

De popup sluit zodra de registratie is voltooid; de nieuwe FastComments-tool verschijnt in de lijst **Tools** met de status **Active**.

#### Maak het beschikbaar

Standaard voegt Moodle nieuwe tools toe aan de lijst "Course tools", maar toont ze niet in de activiteitskiezer. Om FastComments cursusbreed zichtbaar te maken:

1. Klik op het tandwielpictogram op de FastComments-tegel.
2. Kies onder **Tool configuration usage** voor **Show in activity chooser and as a preconfigured tool**.
3. Opslaan.

Docenten kunnen nu FastComments aan elke cursus toevoegen via **Add an activity or resource** > **FastComments**.
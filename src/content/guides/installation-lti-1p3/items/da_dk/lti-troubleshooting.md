#### "Registreringstoken ikke fundet, udløbet eller allerede brugt"

Tokenet i din registrerings-URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">hent den her</a>) er gyldigt i 30 minutter og kan kun bruges én gang. Hvis din LMS tog længere tid end det, eller hvis registreringen blev forsøgt igen efter at være gennemført, vil tokenet blive afvist. Generer en ny URL på FastComments LTI 1.3-konfigurationssiden og start forfra.

#### "Platform rejected registration"

Din LMS afviste registrerings-handshaken. De mest almindelige årsager:

- **Tool already registered with the same client name.** Nogle platforme (især D2L) afviser en anden registrering af "FastComments", indtil den forrige er slettet. Fjern det gamle værktøj i din LMS, og prøv igen.
- **Wrong field in the LMS.** Sørg for, at du indsatte URL'en i feltet **registration / tool initiation registration endpoint**, ikke i launch URL- eller login URL-feltet.
- **The LMS doesn't actually support Dynamic Registration.** Ældre Moodle- og Blackboard-versioner angiver LTI 1.3, men tillader kun manuel konfiguration. Tjek din platforms dokumentation.

#### "Failed to fetch platform configuration"

FastComments kunne ikke læse din LMS' openid-configuration-dokument. Dette er sjældent og betyder normalt, at LMS'en leverede en fejlbehæftet eller utilgængelig discovery-URL. Kontakt din LMS-support.

#### Launch shows "Configuration not found"

Enten blev konfigurationen i FastComments slettet, eller opstarten kom fra et `iss`/`client_id`-par, vi ikke genkender. Hvis du slettede og genregistrerede, bed din LMS om at fjerne og tilføje FastComments-værktøjet igen, så det får det nye client_id.

#### Launch shows "Deployment not registered"

Du startede FastComments fra en Brightspace/Moodle/Blackboard-udrulning, som er forskellig fra den, den først blev startet i. FastComments gemmer `deployment_id` ved første start som en sikkerhedskontrol. For at tilføje en ny deployment under samme klient, kontakt support — vi tilføjer deployment ID til konfigurationen.

#### Launch shows "Unsupported message_type"

LMS'en sendte en LTI-besked, som FastComments ikke håndterer (f.eks. `LtiSubmissionReviewRequest`). FastComments understøtter kun den standard resource-link-opstart og deep-linking-flow. Kontakt os, hvis du har brug for, at en bestemt beskedtype tilføjes.

#### Iframe doesn't resize

De fleste LMS'er ændrer automatisk størrelse på LTI-iframe. Hvis din ikke gør det, så tjek, at LMS'ens opstartsindstillinger tillader værktøjet at sende postMessage-events til forældrerammen. FastComments udsender både Canvas-style (`lti.frameResize`) og IMS-spec (`org.imsglobal.lti.frameResize`) resize-beskeder.
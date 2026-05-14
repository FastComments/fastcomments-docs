#### "Registreringstoken ikke fundet, udløbet eller allerede brugt"

Tokenet i din registrerings-URL er gyldigt i 30 minutter og kan kun bruges én gang. Hvis dit LMS brugte længere tid end det, eller hvis registreringen blev forsøgt igen efter at være lykkedes, vil tokenet blive afvist. Generer en ny URL på FastComments LTI 1.3 Configuration page og start forfra.

#### "Platform rejected registration"

Dit LMS afviste registreringshandshake. De mest almindelige årsager:

- **Tool already registered with the same client name.** Nogle platforme (især D2L) afviser en anden registrering af "FastComments", indtil den tidligere er slettet. Fjern det gamle værktøj i dit LMS, og prøv igen.
- **Wrong field in the LMS.** Sørg for, at du indsatte URL'en i feltet **registration / tool initiation registration endpoint**, ikke i launch URL- eller login URL-feltet.
- **The LMS doesn't actually support Dynamic Registration.** Ældre versioner af Moodle og Blackboard annoncerer LTI 1.3, men tillader kun manuel konfiguration. Tjek din platforms dokumentation.

#### "Failed to fetch platform configuration"

FastComments kunne ikke læse dit LMS' openid-configuration-dokument. Dette er sjældent og betyder normalt, at LMS'et leverede en forkert eller utilgængelig discovery-URL. Kontakt dit LMS-support.

#### Launch shows "Configuration not found"

Enten blev konfigurationen i FastComments slettet, eller lanceringen kom fra et `iss`/`client_id`-par, vi ikke genkender. Hvis du slettede og nyregistrerede, bed dit LMS om at fjerne og genindsætte FastComments-værktøjet, så det får den nye client_id.

#### Launch shows "Deployment not registered"

Du startede FastComments fra en Brightspace/Moodle/Blackboard-deployment, der er forskellig fra den, den først blev startet i. FastComments fastlåser `deployment_id` ved første start som en sikkerhedskontrol. For at tilføje en ny deployment under samme klient, kontakt support — vi tilføjer deployment ID'et til konfigurationen.

#### Launch shows "Unsupported message_type"

LMS'et sendte en LTI-meddelelse, som FastComments ikke håndterer (f.eks. `LtiSubmissionReviewRequest`). FastComments understøtter kun standard resource-link launch og deep-linking flows. Kontakt os, hvis du har brug for, at en specifik meddelelsestype tilføjes.

#### Iframe doesn't resize

De fleste LMS'er ændrer automatisk størrelsen på LTI-iframes. Hvis dit ikke gør, skal du kontrollere, at launchindstillingerne i dit LMS tillader værktøjet at sende postMessage-beskeder til parent-frame. FastComments udsender både Canvas-stil (`lti.frameResize`) og IMS-spec (`org.imsglobal.lti.frameResize`) resize-beskeder.
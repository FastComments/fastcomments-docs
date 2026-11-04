#### "Registreringstoken ikke fundet, udløbet eller allerede brugt"

Tokenet i din registrerings-URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">hent det her</a>) er gyldigt i 30 minutter og kan kun bruges én gang. Hvis din LMS tog længere tid om det, eller hvis registreringen blev forsøgt igen efter at være lykkedes, vil tokenet blive afvist. Generér en ny URL på FastComments LTI 1.3-konfigurationssiden og start forfra.

#### "Platform afviste registrering"

Din LMS nægtede registreringsskakmaten. De mest almindelige årsager:

- **Værktøjet er allerede registreret med samme klientnavn.** Nogle platforme (især D2L) afviser en anden registrering af "FastComments", indtil den forrige er slettet. Fjern det gamle værktøj i din LMS, og prøv igen.
- **Forkert felt i LMS'en.** Sørg for, at du indsatte URL'en i feltet **registration / tool initiation registration endpoint**, ikke i feltet til launch-URL eller login-URL.
- **LMS'en understøtter faktisk ikke Dynamic Registration.** Ældre Moodle- og Blackboard-versioner annoncerer LTI 1.3, men tillader kun manuel konfiguration. Tjek din platforms dokumentation.

#### "Kunne ikke hente platformskonfiguration"

FastComments kunne ikke læse din LMS's openid-configuration-dokument. Dette er sjældent og betyder normalt, at LMS'en angav en misdannet eller utilgængelig discovery-URL. Kontakt din LMS's support.

#### Launch viser "Configuration not found"

Enten er konfigurationen i FastComments blevet slettet, eller også kom launchen fra et `iss`/`client_id`-par, vi ikke genkender. Hvis du har slettet og genregistreret, så instruér din LMS i at fjerne og gen-tilføje FastComments-værktøjet, så det får det nye client_id.

#### Launch viser "Deployment not registered"

Du launchede FastComments fra en Brightspace/Moodle/Blackboard-deployment, der var en anden end den, den først blev launchet i. FastComments fastlåser `deployment_id` ved første launch som en sikkerhedskontrol. For at tilføje en ny deployment under samme klient, kontakt supporten - vi tilføjer deployment-ID'et til konfigurationen.

#### Launch viser "Unsupported message_type"

LMS'en sendte en LTI-besked, som FastComments ikke håndterer (f.eks. `LtiSubmissionReviewRequest`). FastComments understøtter kun standard resource-link-launch og deep-linking-flows. Kontakt os, hvis du har brug for en bestemt beskedtype tilføjet.

#### Iframe ændrer ikke størrelse

De fleste LMS'er auto-tilpasser størrelsen på LTI-iframes. Hvis din ikke gør, så tjek, at LMS'ens launch-indstillinger tillader, at værktøjet kan sende postMessage-events til den overordnede frame. FastComments udsender både Canvas-stil (`lti.frameResize`) og IMS-spec (`org.imsglobal.lti.frameResize`) resize-beskeder.
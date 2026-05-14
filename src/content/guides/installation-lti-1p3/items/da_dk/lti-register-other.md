#### Sakai

Sakai understøtter LTI 1.3-dynamisk registrering på udgivelser med LTI Advantage. Fra **Administrationsarbejdsområdet**:

1. Log ind som Sakai-administrator og åbn **Administrationsarbejdsområdet**.
2. Vælg **Eksterne værktøjer** > **Installer LTI 1.3-værktøj**.
3. Indsæt FastComments-registrerings-URL'en og indsend.
4. Godkend værktøjet, når handshaken er fuldført.

Værktøjet vises derefter under **Eksterne værktøjer** og kan tilføjes til sites af deres vedligeholdere.

#### Schoology

Schoology Enterprise-instanser understøtter LTI 1.3, men tilgængeligheden af dynamisk registrering varierer efter udrulning. Kontakt din Schoology-kontoansvarlige.

Hvis dynamisk registrering ikke er tilgængelig på din Schoology-instans, skal du konfigurere integrationen manuelt ved hjælp af disse endepunkter:

- **OIDC-login-URL**: `https://fastcomments.com/lti/v1p3/login`
- **Target Link-URL**: `https://fastcomments.com/lti/v1p3/launch`
- **URL til Public Keyset (JWKS)**: `https://fastcomments.com/lti/v1p3/jwks`
- **Redirect-URL'er**: `https://fastcomments.com/lti/v1p3/launch`

Når Schoology har givet dig et Client ID og et Deployment ID, kontakt FastComments-support for at registrere konfigurationen på din tenant.

#### Andre LTI 1.3-platforme

Enhver LMS, der følger IMS LTI 1.3 Advantage-specifikationen, bør fungere med den samme registrerings-URL. Kig efter en indstilling mærket "Dynamic Registration", "Tool Registration URL", "Tool initiation registration endpoint", eller lignende.

Hvis din platform kun understøtter manuel LTI 1.3-opsætning, så brug de fire endepunkter angivet i Schoology-sektionen ovenfor og kontakt support for at færdiggøre.
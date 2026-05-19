#### Sakai

Sakai understøtter LTI 1.3 dynamisk registrering på udgivelser med LTI Advantage. Fra administrationsarbejdsområdet:

1. Log ind som Sakai-administrator og åbn **Administrationsarbejdsområdet**.
2. Vælg **Eksterne værktøjer** > **Installer LTI 1.3-værktøj**.
3. Indsæt FastComments-registrerings-URL'en (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">hent den her</a>) og indsend.
4. Godkend værktøjet, når håndtrykket er fuldført.

Værktøjet vises derefter under **Eksterne værktøjer** og kan tilføjes til sites af deres vedligeholdere.

#### Schoology

Schoology Enterprise-instanser understøtter LTI 1.3, men tilgængeligheden af dynamisk registrering varierer efter udrulning. Kontakt din Schoology-kontoansvarlige.

Hvis dynamisk registrering ikke er tilgængelig på din Schoology-instans, skal du konfigurere integrationen manuelt ved hjælp af disse endepunkter:

- **OIDC-login-URL**: `https://fastcomments.com/lti/v1p3/login`
- **Target Link URL**: `https://fastcomments.com/lti/v1p3/launch`
- **Public Keyset URL (JWKS)**: `https://fastcomments.com/lti/v1p3/jwks`
- **Redirect URLs**: `https://fastcomments.com/lti/v1p3/launch`

Efter at Schoology giver dig et Client ID og Deployment ID, kontakt FastComments-support for at registrere konfigurationen på din tenant.

#### Other LTI 1.3 Platforms

Enhver LMS, der følger IMS LTI 1.3 Advantage-specifikationen, bør fungere med den samme registrerings-URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">hent den her</a>). Kig efter en indstilling mærket "Dynamisk registrering", "Tool Registration URL", "Tool initiation registration endpoint" eller lignende.

Hvis din platform kun understøtter manuel LTI 1.3-opsætning, skal du bruge de fire endepunkter, der er angivet i Schoology-afsnittet ovenfor, og kontakte support for at afslutte.
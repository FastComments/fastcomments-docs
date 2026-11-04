#### Sakai

Sakai understøtter LTI 1.3 Dynamic Registration på udgivelser med LTI Advantage. Fra Administration Workspace:

1. Log ind som Sakai-administrator og åbn **Administration Workspace**.
2. Vælg **External Tools** > **Install LTI 1.3 Tool**.
3. Indsæt FastComments-registrerings-URL'en (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">få den her</a>) og indsend.
4. Godkend værktøjet, når håndtrykket er fuldført.

Værktøjet vises derefter under **External Tools** og kan tilføjes til sider af deres vedligeholdere.

#### Schoology

Schoology Enterprise-instanser understøtter LTI 1.3, men tilgængeligheden af Dynamic Registration varierer afhængigt af udrulningen. Tjek med din Schoology-kontoadministrator.

Hvis Dynamic Registration ikke er tilgængelig på din Schoology-instans, skal du konfigurere integrationen manuelt ved hjælp af disse endpoints:

- **OIDC Login URL**: `https://fastcomments.com/lti/v1p3/login`
- **Target Link URL**: `https://fastcomments.com/lti/v1p3/launch`
- **Public Keyset URL (JWKS)**: `https://fastcomments.com/lti/v1p3/jwks`
- **Redirect URLs**: `https://fastcomments.com/lti/v1p3/launch`

Når Schoology har givet dig et Client ID og Deployment ID, skal du kontakte FastComments-support for at registrere konfigurationen på din tenant.

#### Andre LTI 1.3-platforme

Enhver LMS, der følger IMS LTI 1.3 Advantage-specifikationen, bør fungere med den samme registrerings-URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">få den her</a>). Kig efter en indstilling mærket "Dynamic Registration", "Tool Registration URL", "Tool initiation registration endpoint" eller lignende.

Hvis din platform kun understøtter manuel LTI 1.3-opsætning, skal du bruge de fire endpoints, der er angivet i Schoology-sektionen ovenfor, og kontakte support for at afslutte.
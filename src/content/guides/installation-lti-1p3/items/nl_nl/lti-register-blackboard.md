Blackboard Learn SaaS en Ultra ondersteunen LTI 1.3 Dynamic Registration.

#### Open het Tool Provider-scherm

1. Meld u aan bij Blackboard als systeembeheerder.
2. Ga naar **Administrator Panel** > **Integrations** > **LTI Tool Providers**.
3. Klik op **Register LTI 1.3 / LTI Advantage Tool**.

Als u alleen "Register LTI 1.1 Provider" ziet, ondersteunt uw Blackboard-versie LTI 1.3 nog niet - upgrade of neem contact op met Blackboard support.

#### Plak de URL

Plak de FastComments-registratie-URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">haal het hier op</a>) in het veld **Client ID** / **Registration URL** (de benaming in Blackboard varieert per versie). Indienen.

Blackboard voert de registratie-handshake met FastComments uit en toont een bevestigingsscherm.

#### Goedkeuren en inschakelen

Blackboard markeert nieuw-geregistreerde tools standaard als **Approved but excluded**:

1. Zoek de FastComments-vermelding in de tool provider-lijst.
2. Open het menu en kies **Edit**.
3. Stel **Tool Status** in op **Approved**.
4. Controleer onder **Institution Policies** welke gebruikersgegevens worden verzonden (naam, e-mail, rol). Sla op.

De tool is nu beschikbaar voor docenten wanneer zij inhoud aan cursussen toevoegen.
Blackboard Learn SaaS und Ultra unterstützen die dynamische Registrierung von LTI 1.3.

#### Tool Provider-Bildschirm öffnen

1. Melden Sie sich bei Blackboard als Systemadministrator an.
2. Navigieren Sie zu **Administrator Panel** > **Integrations** > **LTI Tool Providers**.
3. Klicken Sie auf **Register LTI 1.3 / LTI Advantage Tool**.

Wenn Sie nur 'Register LTI 1.1 Provider' sehen, unterstützt Ihre Blackboard-Version noch kein LTI 1.3 – aktualisieren Sie oder kontaktieren Sie den Blackboard-Support.

#### URL einfügen

Fügen Sie die FastComments-Registrierungs-URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">hier abrufen</a>) in das Feld **Client ID** / **Registration URL** ein (die Bezeichnungen in Blackboard variieren je nach Version). Absenden.

Blackboard führt den Registrierungs-Handshake mit FastComments durch und zeigt Ihnen eine Bestätigungsseite an.

#### Genehmigen und aktivieren

Blackboard markiert neu registrierte Tools standardmäßig als **Approved but excluded**:

1. Suchen Sie den Eintrag für FastComments in der Tool-Provider-Liste.
2. Öffnen Sie das Menü und wählen Sie **Edit**.
3. Setzen Sie den **Tool Status** auf **Approved**.
4. Überprüfen Sie unter **Institution Policies**, welche Benutzerdaten gesendet werden (Name, E-Mail, Rolle). Speichern.

Das Tool ist jetzt für Lehrende verfügbar, wenn sie Inhalte zu Kursen hinzufügen.
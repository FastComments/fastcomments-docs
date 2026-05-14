#### Sakai

Sakai unterstützt LTI 1.3 Dynamic Registration in Versionen mit LTI Advantage. Öffnen Sie dazu den **Administration Workspace**:

1. Melden Sie sich als Sakai-Administrator an und öffnen Sie den **Administration Workspace**.
2. Wählen Sie **External Tools** > **Install LTI 1.3 Tool**.
3. Fügen Sie die FastComments-Registrierungs-URL ein und senden Sie das Formular ab.
4. Genehmigen Sie das Tool, wenn der Handshake abgeschlossen ist.

Das Tool erscheint dann unter **External Tools** und kann von den Verantwortlichen der Sites hinzugefügt werden.

#### Schoology

Schoology Enterprise-Instanzen unterstützen LTI 1.3, aber die Verfügbarkeit von Dynamic Registration hängt von der jeweiligen Deployment-Konfiguration ab. Klären Sie das mit Ihrem Schoology-Kundenbetreuer.

Wenn Dynamic Registration in Ihrer Schoology-Instanz nicht verfügbar ist, müssen Sie die Integration manuell mit folgenden Endpunkten konfigurieren:

- **OIDC Login URL**: `https://fastcomments.com/lti/v1p3/login`
- **Target Link URL**: `https://fastcomments.com/lti/v1p3/launch`
- **Public Keyset URL (JWKS)**: `https://fastcomments.com/lti/v1p3/jwks`
- **Redirect URLs**: `https://fastcomments.com/lti/v1p3/launch`

Nachdem Schoology Ihnen eine Client ID und eine Deployment ID gegeben hat, kontaktieren Sie den FastComments-Support, um die Konfiguration in Ihrem tenant zu registrieren.

#### Andere LTI 1.3 Plattformen

Jedes LMS, das der IMS LTI 1.3 Advantage-Spezifikation folgt, sollte mit derselben Registrierungs-URL funktionieren. Suchen Sie nach einer Einstellung mit der Bezeichnung "Dynamic Registration", "Tool Registration URL", "Tool initiation registration endpoint" oder ähnlich.

Wenn Ihre Plattform nur die manuelle LTI 1.3 Einrichtung unterstützt, verwenden Sie die vier oben im Schoology-Abschnitt aufgeführten Endpunkte und kontaktieren Sie den Support, um die Einrichtung abzuschließen.
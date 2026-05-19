#### Sakai

Sakai unterstützt LTI 1.3 Dynamische Registrierung in Releases mit LTI Advantage. Aus dem Administration Workspace:

1. Melden Sie sich als Sakai-Administrator an und öffnen Sie den **Administration Workspace**.
2. Wählen Sie **External Tools** > **Install LTI 1.3 Tool**.
3. Fügen Sie die FastComments-Registrierungs-URL ein (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">hier abrufen</a>) und senden Sie sie ab.
4. Genehmigen Sie das Tool, wenn der Handshake abgeschlossen ist.

Das Tool erscheint dann unter **External Tools** und kann von den Betreuern der Sites hinzugefügt werden.

#### Schoology

Schoology Enterprise-Instanzen unterstützen LTI 1.3, aber die Verfügbarkeit der Dynamischen Registrierung variiert je nach Deployment. Klären Sie dies mit Ihrem Schoology-Kundenbetreuer.

Wenn die Dynamische Registrierung in Ihrer Schoology-Instanz nicht verfügbar ist, müssen Sie die Integration manuell mit diesen Endpunkten konfigurieren:

- **OIDC Login URL**: `https://fastcomments.com/lti/v1p3/login`
- **Target Link URL**: `https://fastcomments.com/lti/v1p3/launch`
- **Public Keyset URL (JWKS)**: `https://fastcomments.com/lti/v1p3/jwks`
- **Redirect URLs**: `https://fastcomments.com/lti/v1p3/launch`

Nachdem Schoology Ihnen eine Client ID und eine Deployment ID gegeben hat, kontaktieren Sie den FastComments-Support, um die Konfiguration in Ihrem Mandanten zu registrieren.

#### Andere LTI 1.3-Plattformen

Jedes LMS, das der IMS LTI 1.3 Advantage-Spezifikation folgt, sollte mit derselben Registrierungs-URL funktionieren (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">hier abrufen</a>). Suchen Sie nach einer Einstellung mit der Bezeichnung "Dynamic Registration", "Tool Registration URL", "Tool initiation registration endpoint" oder ähnlichem.

Wenn Ihre Plattform nur eine manuelle LTI 1.3-Konfiguration unterstützt, verwenden Sie die vier oben im Schoology-Abschnitt aufgeführten Endpunkte und kontaktieren Sie den Support, um die Einrichtung abzuschließen.
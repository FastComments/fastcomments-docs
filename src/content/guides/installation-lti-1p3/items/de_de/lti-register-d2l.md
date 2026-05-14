D2L Brightspace stellt die dynamische Registrierung über die LTI Advantage-Administrationsoberfläche bereit. Sie benötigen Administratorzugriff.

#### Registrierungsbildschirm öffnen

1. Melden Sie sich als Administrator in Ihrer Brightspace-Instanz an.
2. Navigieren Sie zu **Admin Tools** > **Manage Extensibility** > **LTI Advantage**.
3. Klicken Sie auf **Register Tool**. (Die direkte URL ist `https://<your-brightspace-host>/d2l/le/ltiadvantage/registrations/create`.)

#### URL einfügen

Sie sehen ein Registrierungsformular. Das wichtigste Feld ist **Tool initiation registration endpoint** (einige Brightspace-Versionen bezeichnen es als "Tool Initiation Registration URL").

Fügen Sie die FastComments-Registrierungs-URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">hier abrufen</a>) in dieses Feld ein. Lassen Sie die anderen Felder leer – sie werden während des Registrierungs-Handshakes von FastComments automatisch ausgefüllt.

Klicken Sie auf **Register**.

#### Tool genehmigen

Brightspace öffnet ein Popup, das mit FastComments kommuniziert, Schlüssel austauscht und einen Bestätigungsbildschirm anzeigt. Das Popup schließt sich, wenn die Registrierung abgeschlossen ist.

Das neue Tool erscheint in Ihrer LTI Advantage-Toolliste. Standardmäßig markiert Brightspace neue Tools als **disabled** – schalten Sie den Schalter auf **enabled**, damit Ihre Kurse es verwenden können.

#### Bereitstellung hinzufügen

In Brightspace benötigen LTI-Tools eine **deployment**, bevor sie in Kursen verwendet werden können:

1. Öffnen Sie das neu registrierte FastComments-Tool.
2. Klicken Sie auf **View Deployments** > **New Deployment**.
3. Geben Sie der Bereitstellung einen Namen (z. B. "FastComments - All Courses"), wählen Sie die Organisations-Einheiten aus, in denen sie verfügbar sein soll, und speichern Sie.

Nach dem ersten Start über diese Bereitstellung pinnt FastComments die `deployment_id` in seinen Konfigurationsdatensatz – nachfolgende Starts von einer anderen Bereitstellung unter demselben Client werden abgelehnt, es sei denn, Sie registrieren erneut.